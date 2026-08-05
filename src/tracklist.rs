// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Disc {
	pub tracks: Vec<Track>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
	pub title: String
}
