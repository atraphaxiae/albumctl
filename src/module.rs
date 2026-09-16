// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct RootModule {
	pub output_directory: PathBuf,
	pub modules: Vec<PathBuf>,

	#[serde(flatten)]
	pub fields: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Module {
	pub children: Children,

	#[serde(flatten)]
	pub fields: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<PathBuf> },
}
