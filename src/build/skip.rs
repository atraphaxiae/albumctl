// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::{BuildIndex, unit::Unit},
	manifest::save_manifest,
	result::Result,
};

pub fn skip_units<'a>(
	units: Vec<Unit<'a>>,
	previous_index: &BuildIndex,
	current_index: &mut BuildIndex,
	index_file: &Path,
) -> Vec<Unit<'a>> {
	let mut units_to_build = Vec::new();
	for unit in units {
		let hash = unit.hash.to_string();
		if let Some(unit_output_files) = previous_index.get(&hash) {
			let skip = unit_output_files
				.iter()
				.all(|file| match file.try_exists() {
					// If we don't know whether the files exist or not, don't trigger a rebuild
					Ok(true) => true,
					Err(_) => true,
					Ok(false) => false,
				});

			if skip {
				if let Err(e) = skip_unit(&unit, unit_output_files, current_index, index_file) {
					eprintln!("{e:?}\n");
				}
			} else {
				units_to_build.push(unit);
			}
		} else {
			units_to_build.push(unit);
		}
	}

	units_to_build
}

fn skip_unit(
	unit: &Unit,
	unit_output_files: &[PathBuf],
	current_index: &mut BuildIndex,
	index_file: &Path,
) -> Result<(), SkipError> {
	let error = || SkipError {
		dir: unit.unit_dir.to_path_buf(),
	};

	current_index.insert(unit.hash.to_string(), unit_output_files.to_vec());
	save_manifest(index_file, current_index).change_context_lazy(error)?;

	Ok(())
}

#[derive(Debug, Error)]
#[error("Could not skip unit {dir:?}")]
struct SkipError {
	dir: PathBuf,
}
