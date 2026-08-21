// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Build module for processed units
//!
//! This is where we finally process the normalized files. For now, we're not gonna do anything to
//! them yet.

use std::path::PathBuf;

use blake3::Hash;

use crate::{
	build::{UnitTrack, normalize::NormalizedUnit},
	model::{AlbumInfo, Config, ReleaseInfo},
};

#[derive(Debug)]
pub struct ProcessedUnit<'a> {
	pub dir: PathBuf,
	pub config: &'a Config,
	pub album_info: &'a AlbumInfo,
	pub release_info: &'a ReleaseInfo,
	pub tracks: Vec<UnitTrack<'a>>,
	pub hash: Hash,
}

impl<'a> ProcessedUnit<'a> {
	pub fn new(unit: NormalizedUnit<'a>) -> Self {
		Self {
			dir: unit.dir,
			config: unit.config,
			album_info: unit.album_info,
			release_info: unit.release_info,
			tracks: unit.tracks,
			hash: unit.hash,
		}
	}
}
