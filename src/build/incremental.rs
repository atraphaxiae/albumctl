// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::{
	build::prepare::BuildIndex,
	module::{File, Metadata},
	result::Result,
};

pub fn incremental_build(
	module_dir: &Path,
	metadata: &Metadata,
	files: &[File],
	previous_index: &BuildIndex,
	current_index: &mut BuildIndex,
	index_file: &Path,
	total_units: &mut usize,
	successful_units: &mut usize,
) -> Result<(), IncrementalBuildError> {
	*total_units += 1;
	// TODO!
	println!("{metadata:#?}");
	*successful_units += 1;

	Ok(())
}

#[derive(Debug, Error)]
#[error("Could not build unit {dir:?}")]
pub struct IncrementalBuildError {
	dir: PathBuf,
}
