// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Module {
	fields: HashMap<String, String>,
	children: Children,
}

#[derive(Debug)]
enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<PathBuf> },
}
