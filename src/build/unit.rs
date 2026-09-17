// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	filesystem::{copy_file, ensure_dir},
	module::{Disc, File, Metadata},
	result::Result,
};

pub fn build_unit(
	hash: &Hash,
	module_dir: &Path,
	metadata: &Metadata,
	tracklist: &[Disc],
	files: &[File],
	output_dir: &Path,
) -> Result<PathBuf, UnitBuildError> {
	let error = || UnitBuildError {
		dir: module_dir.to_path_buf(),
	};

	let unit_build_dir = output_dir.join(format!(".albumctl/{}", hash));
	ensure_dir(&unit_build_dir).change_context_lazy(error)?;

	for file in files {
		let from = module_dir.join(&file.file);
		let to = unit_build_dir.join(&file.file);
		copy_file(&from, &to).change_context_lazy(error)?;
	}

	todo!();
}

#[derive(Debug, Error)]
#[error("Could not build unit {dir:?}")]
pub struct UnitBuildError {
	dir: PathBuf,
}
