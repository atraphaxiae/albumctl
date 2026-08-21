// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	ffi::OsStr,
	path::{Path, PathBuf},
};

use blake3::Hash;
use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
	build::{normalize::NormalizedUnit, prepare::PreparedUnit, raw::RawUnit},
	filesystem::require_file,
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
	pub fn new(model: Model) -> Self {
		Self { model }
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

	pub fn build(&self) -> Result<(), BuildError> {
		let error = || BuildError::Build {
			dir: self.model.dir.clone(),
		};

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

				let unit = PreparedUnit::new(unit).change_context_lazy(error)?;
				let unit = NormalizedUnit::new(unit).change_context_lazy(error)?;

				dbg!(unit);
			}
		}

		Ok(())
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
	#[error("Errors were detected in source directory {dir:?}")]
	Check { dir: PathBuf },

	#[error("Could not build source directory {dir:?}")]
	Build { dir: PathBuf },

	#[error("Could not get filename of {file:?}")]
	GetFileName { file: PathBuf },
}
