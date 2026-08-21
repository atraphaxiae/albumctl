// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	collections::HashMap,
	ffi::OsStr,
	path::{Path, PathBuf},
};

use blake3::Hash;
use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
	build::{
		finalize::FinalizedUnit, normalize::NormalizedUnit, prepare::PreparedUnit,
		process::ProcessedUnit, raw::RawUnit,
	},
	filesystem::{delete_dir, ensure_dir, ensure_file, require_file},
	manifest::{load_manifest, save_manifest},
	model::{AlbumInfo, Config, DiscInfo, Model, ReleaseInfo, TrackInfo},
	result::Result,
};

pub mod finalize;
pub mod normalize;
pub mod prepare;
pub mod process;
pub mod raw;

#[derive(Debug)]
pub struct Builder {
	model: Model,
}

impl Builder {
	pub fn new(model: Model) -> Result<Self, BuildError> {
		let error = || BuildError::NewBuilder {
			dir: model.dir.to_path_buf(),
		};

		Ok(Self { model })
	}

	pub fn check(&self) -> Result<(), BuildError> {
		let error = || BuildError::Check {
			dir: self.model.dir.clone(),
		};

		// Simply check if the hash can be calculated, and if the source files are actually there
		for album in &self.model.albums {
			for release in &album.releases {
				let unit = RawUnit::new(
					&self.model.config,
					&album.info,
					&release.info,
					&release.discs,
					&release.dir,
				)
				.change_context_lazy(error)?;

				for track in unit.tracks {
					require_file(&track.file).change_context_lazy(error)?;
				}
			}
		}

		Ok(())
	}

	pub fn build(&self) -> Result<PathBuf, BuildError> {
		let error = || BuildError::Build {
			dir: self.model.dir.clone(),
		};

		let build_dir = self.model.config.output_dir.join(".albumctl");
		let index_file = build_dir.join("index.toml");
		ensure_dir(&build_dir).change_context_lazy(error)?;
		ensure_file(&index_file, Some("")).change_context_lazy(error)?;

		let mut previous_index = load_manifest::<Index>(&index_file)
			.change_context_lazy(error)?
			.entries
			.into_iter()
			.map(|entry| (entry.hash, entry.dir))
			.collect::<HashMap<_, _>>();

		let mut current_index = Index {
			entries: Vec::new(),
		};

		// Create the raw units. If a raw unit hash is in the previous index, remove it from there
		// and insert it into the current index. If not, push it into raw_units.
		let mut raw_units = Vec::new();
		for album in &self.model.albums {
			for release in &album.releases {
				let raw = RawUnit::new(
					&self.model.config,
					&album.info,
					&release.info,
					&release.discs,
					&release.dir,
				)
				.change_context_lazy(error)?;

				if let Some((hash, dir)) = previous_index.remove_entry(&raw.hash) {
					current_index.entries.push(IndexEntry { dir, hash });
				} else {
					raw_units.push(raw);
				}
			}
		}

		// Delete all directories referenced by the previous index, *then* save the current index.
		// This order is important because if any deletion fails, we can still resume the deletion
		// on the next call of albumctl build since the index file hasn't been changed yet.
		for (_, dir) in previous_index {
			delete_dir(&dir).change_context_lazy(error)?;
		}

		save_manifest(&index_file, &current_index).change_context_lazy(error)?;

		// Finally, build all of the units, writing to the index file with each success.
		// TODO: In the scenario where the unit was successfully finalized, but save_manifest fails,
		// the release is in the output directory but the index does not contain its hash and path.
		// This means that the next build will fail because when finalizing, it will try to copy the
		// files to the already existing output directory.
		for raw in raw_units {
			let prepared = PreparedUnit::new(raw).change_context_lazy(error)?;
			let normalized = NormalizedUnit::new(prepared).change_context_lazy(error)?;
			let processed = ProcessedUnit::new(normalized);
			let finalized = FinalizedUnit::new(processed).change_context_lazy(error)?;

			current_index.entries.push(IndexEntry { dir: finalized.dir, hash: finalized.hash });
			save_manifest(&index_file, &current_index).change_context_lazy(error)?;
		}

		// We're DONE!!
		Ok(self.model.config.output_dir.clone())
	}
}

#[derive(Debug, Serialize)]
pub struct UnitTrack<'a> {
	pub disc_index: usize,
	pub track_index: usize,
	pub disc_info: &'a DiscInfo,
	pub track_info: &'a TrackInfo,
	pub file: PathBuf,
}

impl UnitTrack<'_> {
	pub fn file_name(&self) -> Result<&OsStr, BuildError> {
		let error = || BuildError::GetFileName {
			file: self.file.to_path_buf(),
		};

		let file_name = self
			.file
			.file_name()
			.ok_or_else(error)
			.attach_with(|| format!("the path must not terminate with \"..\""))?;

		Ok(file_name)
	}

	pub fn extension(&self) -> Option<&OsStr> {
		self.file.extension()
	}

	pub fn with_file(mut self, file: &Path) -> Self {
		self.file = file.to_path_buf();
		self
	}
}

#[derive(Debug, Serialize)]
pub struct UnitFingerprint<'a> {
	config: &'a Config,
	album_info: &'a AlbumInfo,
	release_info: &'a ReleaseInfo,
	tracks: &'a [UnitTrack<'a>],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
	entries: Vec<IndexEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
	dir: PathBuf,
	hash: Hash,
}

#[derive(Debug, Error)]
pub enum BuildError {
	#[error("Could not create builder to source directory {dir:?}")]
	NewBuilder { dir: PathBuf },

	#[error("Errors were detected in source directory {dir:?}")]
	Check { dir: PathBuf },

	#[error("Could not build source directory {dir:?}")]
	Build { dir: PathBuf },

	#[error("Could not get filename of {file:?}")]
	GetFileName { file: PathBuf },
}
