// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

use crate::build::{convert::Convert, replaygain::Replaygain};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootModule {
	pub output_directory: PathBuf,
	pub convert: Option<Convert>,
	pub replaygain: Option<Replaygain>,
	pub metadata: Metadata,
	pub children: RootChildren,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootChildren {
	pub modules: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Module {
	pub metadata: Metadata,
	pub discs: Option<Vec<Disc>>,
	pub children: Children,
}

pub type Metadata = BTreeMap<String, String>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Disc {
	pub metadata: Metadata,
	pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
	pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<File> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct File {
	pub file: PathBuf,
	pub disc_number: usize,
	pub track_number: usize,
}
