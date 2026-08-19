// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Module for loading the source directory into source structures
//!
//! Here, we load the manifests referenced by SourceDir into Source. We are not yet concerned with
//! duplicate album/release detection in this layer.

use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug)]
pub struct Source {
	pub dir: PathBuf,
	pub config: ConfigManifest,
	pub albums: Vec<AlbumSource>,
}

#[derive(Debug)]
pub struct AlbumSource {
	pub dir: PathBuf,
	pub manifest: AlbumManifest,
	pub releases: Vec<ReleaseSource>,
}

#[derive(Debug)]
pub struct ReleaseSource {
	pub dir: PathBuf,
	pub manifest: ReleaseManifest,
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
	#[error("Could not load source directory {dir:?}")]
	SourceLoad { dir: PathBuf },

	#[error("Could not load album directory {dir:?}")]
	AlbumLoad { dir: PathBuf },

	#[error("Could not load release directory {dir:?}")]
	ReleaseLoad { dir: PathBuf }
}
