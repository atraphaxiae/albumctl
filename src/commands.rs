// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::Path;

use error_stack::ResultExt;
use thiserror::Error;

use crate::print::success;
use crate::project::Project;
use crate::result::Result;

pub fn init(path: &Path) -> Result<(), CommandError> {
	Project::init(path).change_context(CommandError::Init)?;

	success!("Successfully initialized albumctl project at {}", path.display());
	Ok(())
}

pub fn check(path: &Path) -> Result<(), CommandError> {
	let error = || CommandError::Check;

	let project = Project::load(path).change_context(error())?;
	project.check().change_context(error())?;

	success!("Successfully validated albumctl project at {}", path.display());
	Ok(())
}

pub fn build(path: &Path) -> Result<(), CommandError> {
	todo!();
	Ok(())
}

#[derive(Debug, Error)]
pub enum CommandError {
	#[error("Failed to initialize albumctl project")]
	Init,

	#[error("Failed to validate albumctl project")]
	Check,

	#[error("Failed to build albumctl project")]
	Build,
}
