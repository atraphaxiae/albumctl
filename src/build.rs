// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

pub mod clean;
pub mod convert;
pub mod load;
pub mod replaygain;
pub mod skip;
pub mod summary;
pub mod unit;

use std::{
	collections::HashMap,
	fs::read_dir,
	path::{Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::{
		clean::clean_output_dir, load::load_units, skip::skip_units, summary::summarize_build,
		unit::build_units,
	},
	filesystem::{delete_item, ensure_dir, ensure_file},
	manifest::load_manifest,
	module::RootModule,
	result::Result,
};

pub type BuildIndex = HashMap<String, Vec<PathBuf>>;

pub fn build(source_dir: &Path) -> Result<(), BuildError> {
	let error = || BuildError {
		dir: source_dir.to_path_buf(),
	};

	let root_module = source_dir.join("albumctl.toml");
	let root_module = load_manifest::<RootModule>(&root_module).change_context_lazy(error)?;

	let build_dir = root_module.output_directory.join(".albumctl");
	ensure_dir(&build_dir).change_context_lazy(error)?;

	let index_file = build_dir.join("build.toml");
	ensure_file(&index_file, None).change_context_lazy(error)?;

	let previous_index = load_manifest::<BuildIndex>(&index_file).change_context_lazy(error)?;
	let mut current_index = BuildIndex::new();

	for entry in read_dir(&build_dir).change_context_lazy(error)? {
		let item = entry.change_context_lazy(error)?.path();
		if item != index_file {
			delete_item(&item).change_context_lazy(error)?;
		}
	}

	let (loaded_units, total_modules, loaded_modules) =
		load_units(source_dir, &root_module).change_context_lazy(error)?;

	let total_units = loaded_units.len();
	let units_to_build = skip_units(
		loaded_units,
		&previous_index,
		&mut current_index,
		&index_file,
	);

	let skipped_units = total_units - units_to_build.len();
	let built_units = build_units(
		&units_to_build,
		&mut current_index,
		&index_file,
		&root_module.output_directory,
	);

	let clean_result = clean_output_dir(&current_index, &root_module.output_directory);
	if let Err(e) = &clean_result {
		eprintln!("{e:?}\n");
	}

	summarize_build(
		source_dir,
		&root_module.output_directory,
		total_modules,
		loaded_modules,
		total_units,
		built_units,
		skipped_units,
		clean_result,
	);

	Ok(())
}

pub fn check() {
	todo!();
}

#[derive(Debug, Error)]
#[error("Failed to build source directory {dir:?}")]
pub struct BuildError {
	dir: PathBuf,
}
