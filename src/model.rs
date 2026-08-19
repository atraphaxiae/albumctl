// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Config {
	pub output_dir: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Album {
	pub info: AlbumInfo,
}

#[derive(Debug, Serialize)]
pub struct AlbumInfo {
	pub id: AlbumId,
}

#[derive(Debug, Serialize)]
pub struct AlbumId {
	pub artist: String,
	pub year: u16,
	pub title: String,
}

#[derive(Debug, Serialize)]
pub struct Release {
	pub info: ReleaseInfo,
	pub discs: Vec<Disc>,
}

#[derive(Debug, Serialize)]
pub struct ReleaseInfo {
	pub id: ReleaseId,
}

#[derive(Debug, Serialize)]
pub struct ReleaseId {
	pub year: u16,
	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,
}

#[derive(Debug, Serialize)]
pub struct Disc {
	pub info: DiscInfo,
	pub tracks: Vec<Track>,
}

#[derive(Debug, Serialize)]
pub struct DiscInfo {
	// will be used in the future
}

#[derive(Debug, Serialize)]
pub struct Track {
	pub info: TrackInfo,
	pub file: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct TrackInfo {
	pub title: String,
}
