// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Build module for finalized units
//!
//! This is where we finalize the per-unit build by moving the files from the unit build directory
//! to the appropriate output directory.

use std::path::PathBuf;

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::process::ProcessedUnit,
	filesystem::{ensure_dir, move_file},
	model::{AlbumId, ReleaseId},
	result::Result,
};

#[derive(Debug)]
pub struct FinalizedUnit {
	pub dir: PathBuf,
	pub hash: Hash,
}

impl FinalizedUnit {
	pub fn new(unit: ProcessedUnit<'_>) -> Result<Self, FinalizeError> {
		let error = || FinalizeError::FinalizeUnit {
			album: unit.album_info.id.clone(),
			release: unit.release_info.id.clone(),
		};

		// Move the processed audio files to the correct output directory
		let dir = unit
			.config
			.output_dir
			.join(format!("{}/{}", unit.album_info.id, unit.release_info.id));
		ensure_dir(&dir).change_context_lazy(error)?;

		for track in unit.tracks {
			let filename = track.file_name().change_context_lazy(error)?;
			let file = dir.join(filename);
			move_file(&track.file, &file).change_context_lazy(error)?;
		}

		Ok(Self {
			dir,
			hash: unit.hash,
		})
	}
}

#[derive(Debug, Error)]
pub enum FinalizeError {
	#[error("Could not finalize unit for \"{album} - {release}\"")]
	FinalizeUnit { album: AlbumId, release: ReleaseId },
}
