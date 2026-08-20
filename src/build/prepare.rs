// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use blake3::Hash;

use crate::{
	build::UnitTrack,
	model::{AlbumInfo, Config, ReleaseInfo},
};

#[derive(Debug)]
pub struct PreparedUnit<'a> {
	pub config: &'a Config,
	pub album_info: &'a AlbumInfo,
	pub release_info: &'a ReleaseInfo,
	pub tracks: Vec<UnitTrack<'a>>,
	pub hash: Hash,
}
