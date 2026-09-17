// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct RootModule {
	pub output_directory: PathBuf,
	pub metadata: Metadata,
	pub children: RootChildren,
}

#[derive(Debug, Deserialize)]
pub struct RootChildren {
	pub modules: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Module {
	pub metadata: Metadata,
	pub discs: Option<Vec<Disc>>,
	pub children: Children,
}

pub type Metadata = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Disc {
	pub metadata: Metadata,
	pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
	pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<File> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
	pub file: PathBuf,
	pub disc_number: usize,
	pub track_number: usize,
}
