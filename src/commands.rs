// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::result::Result;

pub fn init(path: &Path) -> Result<(), CommandError> {
	todo!();
	Ok(())
}

pub fn check(path: &Path) -> Result<(), CommandError> {
	todo!();
	Ok(())
}

pub fn build(path: &Path) -> Result<(), CommandError> {
	todo!();
	Ok(())
}

#[derive(Debug, Error)]
pub enum CommandError {
	#[error("Failed to initialize albumctl project at {path:?}")]
	Init { path: PathBuf },

	#[error("Failed to validate albumctl project at {path:?}")]
	Check { path: PathBuf },

	#[error("Failed to build albumctl project at {path:?}")]
	Build { path: PathBuf },
}
