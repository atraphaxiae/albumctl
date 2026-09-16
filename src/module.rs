// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct RootModule {
	pub output_directory: PathBuf,
	pub modules: Vec<PathBuf>,
	pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Module {
	pub metadata: Metadata,
	pub children: Children,
}

pub type Metadata = HashMap<String, String>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<PathBuf> },
}
