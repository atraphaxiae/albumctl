// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Build module for raw units
//!
//! Here we calculate the unit hash, convert the Vec<Disc> representation to Vec<UnitTrack>, and
//! resolve the paths of the audio files relative to the release directory of the unit.

use std::path::Path;

use blake3::{Hash, hash};
use error_stack::ResultExt;
use postcard::to_stdvec;
use thiserror::Error;

use crate::{
	build::{UnitFingerprint, UnitTrack},
	model::{AlbumId, AlbumInfo, Config, Disc, ReleaseId, ReleaseInfo},
	result::Result,
};

#[derive(Debug)]
pub struct RawUnit<'a> {
	config: &'a Config,
	album_info: &'a AlbumInfo,
	release_info: &'a ReleaseInfo,
	tracks: Vec<UnitTrack<'a>>,
	hash: Hash,
}

impl<'a> RawUnit<'a> {
	pub fn new(
		config: &'a Config,
		album_info: &'a AlbumInfo,
		release_info: &'a ReleaseInfo,
		discs: &'a [Disc],
		release_dir: &Path,
	) -> Result<Self, RawError> {
		let error = || RawError::NewUnit {
			album: album_info.id.clone(),
			release: release_info.id.clone(),
		};

		// Create the tracks array from discs, and resolve each track file relative to release_dir
		let mut tracks = Vec::new();
		for (disc_index, disc) in discs.iter().enumerate() {
			for (track_index, track) in disc.tracks.iter().enumerate() {
				tracks.push(UnitTrack {
					disc_index,
					track_index,
					disc_info: &disc.info,
					track_info: &track.info,
					file: release_dir.join(&track.file),
				})
			}
		}

		// Calculate the unit hash
		let fingerprint = UnitFingerprint {
			config,
			album_info,
			release_info,
			tracks: &tracks,
		};

		let fingerprint_bytes = to_stdvec(&fingerprint).change_context_lazy(error)?;
		let hash = hash(&fingerprint_bytes);

		Ok(Self {
			config,
			album_info,
			release_info,
			tracks,
			hash,
		})
	}
}

#[derive(Debug, Error)]
pub enum RawError {
	#[error("Could not create unit for release \"{album} - {release}\"")]
	NewUnit { album: AlbumId, release: ReleaseId },
}
