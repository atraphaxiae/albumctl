// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::result::Result;

pub fn check(dir: &Path) -> Result<(), CommandError> {
	Ok(())
}

pub fn build(dir: &Path) -> Result<(), CommandError> {
	Ok(())
}

#[derive(Debug, Error)]
pub enum CommandError {
	#[error("Errors were detected in source directory {dir:?}")]
	Check { dir: PathBuf },

	#[error("Could not build source directory {dir:?}")]
	Build { dir: PathBuf },
}
