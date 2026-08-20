// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use blake3::Hash;
use serde::{Deserialize, Serialize};

use crate::model::{DiscInfo, Model, TrackInfo};

pub mod finalize;
pub mod normalize;
pub mod prepare;
pub mod process;

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
	pub disc_index: u16,
	pub track_index: u16,
	pub disc_info: &'a DiscInfo,
	pub track_info: &'a TrackInfo,
	pub file: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
	entries: Vec<IndexEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
	dir: PathBuf,
	unit_hash: Hash,
}
