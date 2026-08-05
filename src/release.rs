// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

use crate::album::AlbumIdentifier;
use crate::tracklist::Disc;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseManifest {
	pub parent: AlbumIdentifier,

	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,

	pub discs: Vec<Disc>
}
