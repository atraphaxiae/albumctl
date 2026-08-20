// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::Builder, dir::SourceDir, model::Model, print::success, result::Result, source::Source,
};

pub fn check(dir: &Path) -> Result<(), CommandError> {
	let error = || CommandError::Check {
		dir: dir.to_path_buf(),
	};

	let source_dir = SourceDir::resolve(dir).change_context_lazy(error)?;
	let source = Source::load(&source_dir).change_context_lazy(error)?;
	let model = Model::from_source(source).change_context_lazy(error)?;
	let builder = Builder::new(model);
	builder.check().change_context_lazy(error)?;

	success!("Successfully validated source at {}", dir.display());
	Ok(())
}

pub fn build(dir: &Path) -> Result<(), CommandError> {
	let error = || CommandError::Build {
		dir: dir.to_path_buf(),
	};

	let source_dir = SourceDir::resolve(dir).change_context_lazy(error)?;
	let source = Source::load(&source_dir).change_context_lazy(error)?;
	let model = Model::from_source(source).change_context_lazy(error)?;

	dbg!(model);

	Ok(())
}

#[derive(Debug, Error)]
pub enum CommandError {
	#[error("Failed to validate source directory {dir:?}")]
	Check { dir: PathBuf },

	#[error("Failed to build source directory {dir:?}")]
	Build { dir: PathBuf },
}
