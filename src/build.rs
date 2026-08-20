// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use crate::model::{DiscInfo, Model, TrackInfo};

pub mod finalize;
pub mod normalize;
pub mod prepare;
pub mod process;

pub struct Builder {
	model: Model,
}

impl Builder {
	pub fn new(model: Model) -> Self {
		Self { model }
	}
}

pub struct UnitTrack<'a> {
	pub disc_number: u16,
	pub track_number: u16,
	pub disc_info: &'a DiscInfo,
	pub track_info: &'a TrackInfo,
	pub file: PathBuf,
}
