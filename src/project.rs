// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::result::Result;

#[derive(Debug)]
pub struct Project {
	root: PathBuf,
}

impl Project {
	pub fn init(path: &Path) -> Result<Project, ProjectError> {
		Ok(Project {
			root: path.to_path_buf(),
		})
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectManifest {
	pub output_dir: PathBuf,
}

#[derive(Debug, Error)]
pub enum ProjectError {
	#[error("Failed to initialize albumctl project at {path:?}")]
	Init { path: PathBuf },
}
