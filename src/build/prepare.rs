// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Build module for prepared units
//!
//! Here we simply copy the files from the release directory to the unit build directory.

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::{UnitTrack, raw::RawUnit},
	filesystem::copy_file,
	model::{AlbumId, AlbumInfo, Config, ReleaseId, ReleaseInfo},
	result::Result,
};

#[derive(Debug)]
pub struct PreparedUnit<'a> {
	pub config: &'a Config,
	pub album_info: &'a AlbumInfo,
	pub release_info: &'a ReleaseInfo,
	pub tracks: Vec<UnitTrack<'a>>,
	pub hash: Hash,
}

impl<'a> PreparedUnit<'a> {
	pub fn new(mut unit: RawUnit<'a>) -> Result<Self, PrepareError> {
		let error = || PrepareError::PrepareUnit {
			album: unit.album_info.id.clone(),
			release: unit.release_info.id.clone(),
		};

		let unit_dir = unit
			.config
			.output_dir
			.join(format!(".albumctl/{}", unit.hash));

		// Copy files from the release directory to the unit build directory, while also updating
		// the path stored in UnitTrack
		let mut tracks = Vec::new();
		for track in unit.tracks {
			let filename = track.file_name().change_context_lazy(error)?;
			let file = unit_dir.join(filename);
			copy_file(&track.file, &file).change_context_lazy(error)?;

			let track = track.with_file(&file);
			tracks.push(track);
		}

		Ok(Self {
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
	#[error("Could not prepare release \"{album} - {release}\"")]
	PrepareUnit { album: AlbumId, release: ReleaseId },
}
