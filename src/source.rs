// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug)]
pub struct Source {}

#[derive(Debug)]
pub struct AlbumSource {}

#[derive(Debug)]
pub struct ReleaseSource {}

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
