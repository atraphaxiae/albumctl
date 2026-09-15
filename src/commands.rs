// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::result::Result;

pub fn build(dir: &Path) -> Result<(), CommandError> {
	todo!()
}

pub fn check(dir: &Path) -> Result<(), CommandError> {
	todo!()
}

#[derive(Debug, Error)]
pub enum CommandError {
	#[error("Could not build source directory {dir:?}")]
	Build { dir: PathBuf },

	#[error("Could not check source directory {dir:?}")]
	Check { dir: PathBuf },
}
