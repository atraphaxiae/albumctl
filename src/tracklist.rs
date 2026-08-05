// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Disc {
	pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
	pub title: String,
}
