// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{collections::HashMap, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Module {
	#[serde(flatten)]
	fields: HashMap<String, String>,
	children: Children,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<PathBuf> },
}
