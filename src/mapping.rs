// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MappingManifest {
	pub discs: Vec<MappingDisc>
}

#[derive(Debug, Deserialize)]
pub struct MappingDisc {
	pub tracks: Vec<MappingTrack>
}

#[derive(Debug, Deserialize)]
pub struct MappingTrack {
	pub file: PathBuf,
}
