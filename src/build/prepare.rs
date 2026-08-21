// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Build module for prepared units
//!
//! Here we simply copy the source audio files to the unit build directory.

use std::path::PathBuf;

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::{UnitTrack, raw::RawUnit},
	filesystem::{copy_file, delete_dir, ensure_dir},
	model::{AlbumId, AlbumInfo, Config, ReleaseId, ReleaseInfo},
	result::Result,
};

#[derive(Debug)]
pub struct PreparedUnit<'a> {
	pub dir: PathBuf,
	pub config: &'a Config,
	pub album_info: &'a AlbumInfo,
	pub release_info: &'a ReleaseInfo,
	pub tracks: Vec<UnitTrack<'a>>,
	pub hash: Hash,
}

impl<'a> PreparedUnit<'a> {
	pub fn new(unit: RawUnit<'a>) -> Result<Self, PrepareError> {
		let error = || PrepareError::PrepareUnit {
			album: unit.album_info.id.clone(),
			release: unit.release_info.id.clone(),
		};

		// Delete the unit build directory from the previous build. Copy the source audio files to
		// the unit build directory, while also updating the path stored in UnitTrack
		delete_dir(&unit.dir).change_context_lazy(error)?;
		ensure_dir(&unit.dir).change_context_lazy(error)?;

		let mut tracks = Vec::new();
		for track in unit.tracks {
			let filename = track.file_name().change_context_lazy(error)?;
			let file = unit.dir.join(filename);
			copy_file(&track.file, &file).change_context_lazy(error)?;

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
pub enum PrepareError {
	#[error("Could not prepare unit for \"{album} - {release}\"")]
	PrepareUnit { album: AlbumId, release: ReleaseId },
}
