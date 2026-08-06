// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::Path;

use error_stack::ResultExt;
use thiserror::Error;

use crate::print::success;
use crate::source::Source;
use crate::result::Result;

pub fn init(path: &Path) -> Result<(), CommandError> {
	Source::init(path).change_context(CommandError::Init)?;

	success!(
		"Successfully initialized albumctl source at {}",
		path.display()
	);
	Ok(())
}

pub fn check(path: &Path) -> Result<(), CommandError> {
	let error = || CommandError::Check;

	let source = Source::load(path).change_context(error())?;
	source.check().change_context(error())?;

	success!(
		"Successfully validated albumctl source at {}",
		path.display()
	);
	Ok(())
}

pub fn build(path: &Path) -> Result<(), CommandError> {
	let error = || CommandError::Build;

	let source = Source::load(path).change_context(error())?;
	let outdir = source.build().change_context(error())?;

	success!(
		"Successfully built albumctl source at {}.\nOutput: {}",
		path.display(),
		outdir.display()
	);
	Ok(())
}

#[derive(Debug, Error)]
pub enum CommandError {
	#[error("Failed to initialize albumctl source")]
	Init,

	#[error("Failed to validate albumctl source")]
	Check,

	#[error("Failed to build albumctl source")]
	Build,
}
