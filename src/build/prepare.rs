// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	filesystem::{ensure_dir, ensure_file},
	manifest::load_manifest,
	module::RootModule,
	result::Result,
};

pub type BuildIndex = HashMap<Hash, PathBuf>;

pub fn build(dir: &Path) -> Result<(), PrepareError> {
	let error = || PrepareError::Build {
		dir: dir.to_path_buf(),
	};

	let root_module = dir.join("albumctl.toml");
	let root_module = load_manifest::<RootModule>(&root_module).change_context_lazy(error)?;
	ensure_dir(&root_module.output_directory).change_context_lazy(error)?;

	let build_dir = root_module.output_directory.join(".albumctl");
	ensure_dir(&build_dir).change_context_lazy(error)?;

	let build_index = build_dir.join(".albumctl/build.toml");
	ensure_file(&build_index, None).change_context_lazy(error)?;

	let previous_build = load_manifest::<BuildIndex>(&build_index).change_context_lazy(error)?;
	let current_index = BuildIndex::new();

	Ok(())
}

pub fn check(dir: &Path) -> Result<(), PrepareError> {
	todo!()
}

#[derive(Debug, Error)]
pub enum PrepareError {
	#[error("Could not build source directory {dir:?}")]
	Build { dir: PathBuf },

	#[error("Could not check source directory {dir:?}")]
	Check { dir: PathBuf },
}
