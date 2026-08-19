// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Module for loading the source directory into source structures
//!
//! Here, we load the manifest files referenced by SourceDir into Source. We are not yet concerned
//! with duplicate album/release detection in this layer.
//!
//! We also define the manifests here that Serde will use for deserializing. This is not yet the
//! final form of the data, as we need to strip some Serde attributes (e.g. `serde(flatten)` or
//! `serde(tag = ...)`) for the data to be usable in the preparation and build process.

use std::path::PathBuf;

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::{
	dir::{AlbumDir, ReleaseDir, SourceDir},
	manifest::load_manifest,
	result::Result,
};

#[derive(Debug)]
pub struct Source {
	pub dir: PathBuf,
	pub config: ConfigManifest,
	pub albums: Vec<AlbumSource>,
}

impl Source {
	pub fn load(dir: &SourceDir) -> Result<Self, SourceError> {
		let error = || SourceError::SourceLoad {
			dir: dir.dir.clone(),
		};

		Ok(Self {
			dir: dir.dir.clone(),
			config: load_manifest(&dir.config_file).change_context_lazy(error)?,
			albums: dir
				.albums
				.iter()
				.map(|album_dir| AlbumSource::load(album_dir))
				.collect::<Result<_, _>>()?,
		})
	}
}

#[derive(Debug)]
pub struct AlbumSource {
	pub dir: PathBuf,
	pub manifest: AlbumManifest,
	pub releases: Vec<ReleaseSource>,
}

impl AlbumSource {
	pub fn load(dir: &AlbumDir) -> Result<Self, SourceError> {
		let error = || SourceError::AlbumLoad {
			dir: dir.dir.clone(),
		};

		Ok(Self {
			dir: dir.dir.clone(),
			manifest: load_manifest(&dir.manifest_file).change_context_lazy(error)?,
			releases: dir
				.releases
				.iter()
				.map(|release_dir| ReleaseSource::load(release_dir))
				.collect::<Result<_, _>>()?,
		})
	}
}

#[derive(Debug)]
pub struct ReleaseSource {
	pub dir: PathBuf,
	pub manifest: ReleaseManifest,
}

impl ReleaseSource {
	pub fn load(dir: &ReleaseDir) -> Result<Self, SourceError> {
		let error = || SourceError::ReleaseLoad {
			dir: dir.dir.clone(),
		};

		Ok(Self {
			dir: dir.dir.clone(),
			manifest: load_manifest(&dir.manifest_file).change_context_lazy(error)?,
		})
	}
}

#[derive(Debug, Deserialize)]
pub struct ConfigManifest {
	pub output_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct AlbumManifest {
	pub artist: String,
	pub year: u16,
	pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseManifest {
	pub year: u16,
	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,

	pub discs: Vec<DiscManifest>,
}

#[derive(Debug, Deserialize)]
pub struct DiscManifest {
	pub tracks: Vec<TrackManifest>,
}

#[derive(Debug, Deserialize)]
pub struct TrackManifest {
	pub title: String,
	pub file: PathBuf,
}

#[derive(Debug, Error)]
pub enum SourceError {
	#[error("Could not load the source directory {dir:?}")]
	SourceLoad { dir: PathBuf },

	#[error("Could not load the album directory {dir:?}")]
	AlbumLoad { dir: PathBuf },

	#[error("Could not load the release directory {dir:?}")]
	ReleaseLoad { dir: PathBuf },
}
