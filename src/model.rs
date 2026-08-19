// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Module for converting source structures into the model structures
//!
//! Here, we finally define the final structures that will own all the manifest data of the source.
//! We consume the manifests here and rearrange their contents into a structure that makes more
//! sense for the prepare and build process.
//!
//! We are also avoiding any Serde attributes by having new structs for the data. We want this
//! because Serde attributes such as `serde(flatten)` or `serde(tag = ...)` don't work with binary
//! serializers, which we need to obtain a stable hash for use with the incremental build process.
//!
//! The model structures are the final structures which will own all of the manifest data. Anything
//! from the prepare and build process will only have references to the data owned by the models.

use std::path::PathBuf;

use serde::Serialize;

use crate::source::{AlbumManifest, ConfigManifest, DiscManifest, ReleaseManifest, TrackManifest};

#[derive(Debug)]
pub struct Model {
	pub dir: PathBuf,
	pub config: Config,
	pub albums: Vec<AlbumModel>,
}

#[derive(Debug)]
pub struct AlbumModel {
	pub dir: PathBuf,
	pub info: AlbumInfo,
	pub releases: Vec<ReleaseModel>,
}

#[derive(Debug)]
pub struct ReleaseModel {
	pub dir: PathBuf,
	pub info: ReleaseInfo,
	pub discs: Vec<Disc>,
}

#[derive(Debug, Serialize)]
pub struct Config {
	pub output_dir: PathBuf,
}

impl Config {
	pub fn from_manifest(manifest: ConfigManifest) -> Self {
		Self {
			output_dir: manifest.output_dir,
		}
	}
}

#[derive(Debug)]
pub struct Album {
	pub info: AlbumInfo,
}

impl Album {
	pub fn from_manifest(manifest: AlbumManifest) -> Self {
		Self {
			info: AlbumInfo {
				id: AlbumId {
					artist: manifest.artist,
					year: manifest.year,
					title: manifest.title,
				},
			},
		}
	}
}

#[derive(Debug, Serialize)]
pub struct AlbumInfo {
	pub id: AlbumId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct AlbumId {
	pub artist: String,
	pub year: u16,
	pub title: String,
}

#[derive(Debug)]
pub struct Release {
	pub info: ReleaseInfo,
	pub discs: Vec<Disc>,
}

impl Release {
	pub fn from_manifest(manifest: ReleaseManifest) -> Self {
		Self {
			info: ReleaseInfo {
				id: ReleaseId {
					year: manifest.year,
					catalog_number: manifest.catalog_number,
					media_type: manifest.media_type,
					audio_channels: manifest.audio_channels,
					provenance: manifest.provenance,
				},
			},

			discs: manifest
				.discs
				.into_iter()
				.map(Disc::from_manifest)
				.collect(),
		}
	}
}

#[derive(Debug, Serialize)]
pub struct ReleaseInfo {
	pub id: ReleaseId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ReleaseId {
	pub year: u16,
	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,
}

#[derive(Debug)]
pub struct Disc {
	pub info: DiscInfo,
	pub tracks: Vec<Track>,
}

impl Disc {
	pub fn from_manifest(manifest: DiscManifest) -> Self {
		Self {
			info: DiscInfo {},
			tracks: manifest
				.tracks
				.into_iter()
				.map(Track::from_manifest)
				.collect(),
		}
	}
}

#[derive(Debug, Serialize)]
pub struct DiscInfo {
	// will be used in the future
}

#[derive(Debug)]
pub struct Track {
	pub info: TrackInfo,
	pub file: PathBuf,
}

impl Track {
	pub fn from_manifest(manifest: TrackManifest) -> Self {
		Self {
			info: TrackInfo {
				title: manifest.title,
			},

			file: manifest.file,
		}
	}
}

#[derive(Debug, Serialize)]
pub struct TrackInfo {
	pub title: String,
}
