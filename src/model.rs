// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Module for converting source structures into the model structures
//!
//! Here, we define the model structures that will own all the manifest data of the source.
//! We consume the manifests here and rearrange their contents into a structure that makes more
//! sense for the prepare and build process. We also finally do duplicate album/release detection
//! here.
//!
//! We are also avoiding any Serde attributes by having new structs for the data. We want this
//! because Serde attributes such as `serde(flatten)` or `serde(tag = ...)` don't work with binary
//! serializers, which we need to obtain a stable hash for use with the incremental build process.
//!
//! The model structures are the final structures which will own all of the manifest data. Anything
//! from the prepare and build process will only have references to the data owned by the models.
//!
//! Note that the `Hash` trait implementations for `AlbumModel` and `ReleaseModel` are only used
//! for duplicate album/release detection, and should not be confused with the stable hash
//! implementation.

use std::{
	cmp::Ordering,
	collections::HashSet,
	fmt::{self, Display, Formatter},
	hash::{Hash, Hasher},
	path::PathBuf,
};

use error_stack::ResultExt;
use indoc::formatdoc;
use serde::Serialize;
use thiserror::Error;

use crate::{
	result::Result,
	source::{
		AlbumManifest, AlbumSource, ConfigManifest, DiscManifest, ReleaseManifest, ReleaseSource,
		Source, TrackManifest,
	},
};

#[derive(Debug)]
pub struct Model {
	pub dir: PathBuf,
	pub config: Config,
	pub albums: Vec<AlbumModel>,
}

impl Model {
	pub fn from_source(source: Source) -> Result<Self, ModelError> {
		let dir = source.dir.to_path_buf();
		let error = || ModelError::ModelSource { dir: dir.clone() };

		let config = Config::from_manifest(source.config);

		// Error on duplicate albums
		let mut albums = HashSet::<AlbumModel>::new();
		for album_source in source.albums {
			let album = AlbumModel::from_source(album_source)?;
			if let Some(existing) = albums.get(&album) {
				Err(error()).attach(formatdoc! {
					"
						duplicate albums of \"{}\" found at:
							- {}
							- {}
					",
					album.info.id,
					existing.dir.display(),
					album.dir.display(),
				})?;
			}

			albums.insert(album);
		}

		let mut albums = albums.into_iter().collect::<Vec<_>>();
		albums.sort();

		Ok(Self {
			dir,
			config,
			albums,
		})
	}
}

#[derive(Debug)]
pub struct AlbumModel {
	pub dir: PathBuf,
	pub info: AlbumInfo,
	pub releases: Vec<ReleaseModel>,
}

impl AlbumModel {
	pub fn from_source(source: AlbumSource) -> Result<Self, ModelError> {
		let dir = source.dir.to_path_buf();
		let error = || ModelError::ModelAlbum { dir: dir.clone() };

		let album = Album::from_manifest(source.manifest);
		let info = album.info;

		// Error on duplicate releases
		let mut releases = HashSet::<ReleaseModel>::new();
		for release_source in source.releases {
			let release = ReleaseModel::from_source(release_source);
			if let Some(existing) = releases.get(&release) {
				Err(error()).attach(formatdoc! {
					"
						duplicate releases of \"{}\" found at:
							- {}
							- {}
					",
					release.info.id,
					existing.dir.display(),
					release.dir.display()
				})?;
			}

			releases.insert(release);
		}

		let mut releases = releases.into_iter().collect::<Vec<_>>();
		releases.sort();

		Ok(Self {
			dir,
			info,
			releases,
		})
	}
}

impl PartialEq for AlbumModel {
	fn eq(&self, other: &Self) -> bool {
		self.info.id == other.info.id
	}
}

impl Eq for AlbumModel {}

impl PartialOrd for AlbumModel {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.info.id.partial_cmp(&other.info.id)
	}
}

impl Ord for AlbumModel {
	fn cmp(&self, other: &Self) -> Ordering {
		self.info.id.cmp(&other.info.id)
	}
}

impl Hash for AlbumModel {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.info.id.hash(state);
	}
}

#[derive(Debug)]
pub struct ReleaseModel {
	pub dir: PathBuf,
	pub info: ReleaseInfo,
	pub discs: Vec<Disc>,
}

impl ReleaseModel {
	pub fn from_source(source: ReleaseSource) -> Self {
		let release = Release::from_manifest(source.manifest);
		Self {
			dir: source.dir,
			info: release.info,
			discs: release.discs,
		}
	}
}

impl PartialEq for ReleaseModel {
	fn eq(&self, other: &Self) -> bool {
		self.info.id == other.info.id
	}
}

impl Eq for ReleaseModel {}

impl PartialOrd for ReleaseModel {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.info.id.partial_cmp(&other.info.id)
	}
}

impl Ord for ReleaseModel {
	fn cmp(&self, other: &Self) -> Ordering {
		self.info.id.cmp(&other.info.id)
	}
}

impl Hash for ReleaseModel {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.info.id.hash(state);
	}
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct AlbumId {
	pub artist: String,
	pub year: u16,
	pub title: String,
}

impl Display for AlbumId {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{} - ({}) {}", self.artist, self.year, self.title)
	}
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ReleaseId {
	pub year: u16,
	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,
}

impl Display for ReleaseId {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"({}) {} [{}, {}, {}]",
			self.year, self.catalog_number, self.media_type, self.audio_channels, self.provenance
		)
	}
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

#[derive(Debug, Error)]
pub enum ModelError {
	#[error("Could not model the source directory {dir:?}")]
	ModelSource { dir: PathBuf },

	#[error("Could not model the album directory {dir:?}")]
	ModelAlbum { dir: PathBuf },
}
