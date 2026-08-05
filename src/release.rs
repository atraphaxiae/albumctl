// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

use crate::tracklist::Disc;

#[derive(Debug, Deserialize)]
pub struct ReleaseManifest {
	#[serde(flatten)]
	pub id: ReleaseIdentifier,

	pub discs: Vec<Disc>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseIdentifier {
	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,
}
