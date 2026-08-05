// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AlbumManifest {
	#[serde(flatten)]
	pub id: AlbumIdentifier,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct AlbumIdentifier {
	pub title: String,
	pub artist: String,
	pub year: u16,
}
