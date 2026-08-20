// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use blake3::Hash;
use serde::{Deserialize, Serialize};

use crate::model::{AlbumInfo, Config, DiscInfo, Model, ReleaseInfo, TrackInfo};

pub mod finalize;
pub mod normalize;
pub mod prepare;
pub mod process;
pub mod raw;

#[derive(Debug)]
pub struct Builder {
	model: Model,
}

impl Builder {
	pub fn new(model: Model) -> Self {
		Self { model }
	}
}

#[derive(Debug, Serialize)]
pub struct UnitTrack<'a> {
	pub disc_index: usize,
	pub track_index: usize,
	pub disc_info: &'a DiscInfo,
	pub track_info: &'a TrackInfo,
	pub file: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct UnitFingerprint<'a> {
	config: &'a Config,
	album_info: &'a AlbumInfo,
	release_info: &'a ReleaseInfo,
	tracks: &'a [UnitTrack<'a>],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
	entries: Vec<IndexEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
	dir: PathBuf,
	hash: Hash,
}
