// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	filesystem::ensure_dir,
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
	todo!();
}

#[derive(Debug, Error)]
#[error("Could not build unit {dir:?}")]
pub struct UnitBuildError {
	dir: PathBuf,
}
