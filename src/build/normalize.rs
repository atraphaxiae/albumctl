// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Build module for normalized units
//!
//! Here we convert the prepared audio files to a consistent format; in our case, this is a single
//! cut audio file per track. Since we're only operating on cut files already for this version of
//! `albumctl`, the only thing this layer does is rename the files to a consistent filename format,
//! i.e. `{disc_number}.{track_number:02} {title}`.

use std::path::PathBuf;

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::{UnitTrack, prepare::PreparedUnit},
	filesystem::move_file,
	model::{AlbumId, AlbumInfo, Config, ReleaseId, ReleaseInfo},
	result::Result,
};

#[derive(Debug)]
pub struct NormalizedUnit<'a> {
	pub dir: PathBuf,
	pub config: &'a Config,
	pub album_info: &'a AlbumInfo,
	pub release_info: &'a ReleaseInfo,
	pub tracks: Vec<UnitTrack<'a>>,
	pub hash: Hash,
}

impl<'a> NormalizedUnit<'a> {
	pub fn new(unit: PreparedUnit<'a>) -> Result<Self, NormalizeError> {
		let error = || NormalizeError::NormalizeUnit {
			album: unit.album_info.id.clone(),
			release: unit.release_info.id.clone(),
		};

		// Rename the prepared tracks to the right format
		let mut tracks = Vec::new();
		for track in unit.tracks {
			let filename = format!(
				"{}.{:02} {}",
				track.disc_index + 1,
				track.track_index + 1,
				track.track_info.title
			);

			let file = unit
				.dir
				.join(filename)
				.with_added_extension(track.extension().unwrap_or_default());

			move_file(&track.file, &file).change_context_lazy(error)?;

			let track = track.with_file(&file);
			tracks.push(track);
		}

		Ok(Self {
			dir: unit.dir,
			config: unit.config,
			album_info: unit.album_info,
			release_info: unit.release_info,
			tracks,
			hash: unit.hash,
		})
	}
}

#[derive(Debug, Error)]
pub enum NormalizeError {
	#[error("Could not normalize unit for \"{album} - {release}\"")]
	NormalizeUnit { album: AlbumId, release: ReleaseId },
}
