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
	build::incremental::incremental_build,
	filesystem::{ensure_dir, ensure_file},
	manifest::load_manifest,
	module::{Children, Metadata, Module, RootModule},
	result::Result,
};

pub type BuildIndex = HashMap<Hash, PathBuf>;

pub fn build(dir: &Path) -> Result<(), PrepareBuildError> {
	let error = || PrepareBuildError::Build {
		dir: dir.to_path_buf(),
	};

	let root_module = dir.join("albumctl.toml");
	let root_module = load_manifest::<RootModule>(&root_module).change_context_lazy(error)?;
	ensure_dir(&root_module.output_directory).change_context_lazy(error)?;

	let build_dir = root_module.output_directory.join(".albumctl");
	ensure_dir(&build_dir).change_context_lazy(error)?;

	let index_file = build_dir.join("build.toml");
	ensure_file(&index_file, None).change_context_lazy(error)?;

	let previous_index = load_manifest::<BuildIndex>(&index_file).change_context_lazy(error)?;
	let mut current_index = BuildIndex::new();

	let mut total_modules = 0_usize;
	let mut total_units = 0_usize;
	let mut successful_modules = 0_usize;
	let mut successful_units = 0_usize;

	for child_dir in root_module.children.modules {
		let child_dir = dir.join(child_dir);
		if let Err(e) = recurse_modules(
			&child_dir,
			&root_module.metadata,
			&previous_index,
			&mut current_index,
			&index_file,
			&mut total_modules,
			&mut total_units,
			&mut successful_modules,
			&mut successful_units,
		) {
			eprintln!("{e:?}");
		}
	}

	println!("Building source directory {dir:?} completed.");
	println!("{successful_modules}/{total_modules} discovered modules loaded successfully.");
	println!("{successful_units}/{total_units} discovered units built successfully.");

	Ok(())
}

pub fn check(dir: &Path) -> Result<(), PrepareBuildError> {
	todo!()
}

fn recurse_modules(
	module_dir: &Path,
	parent_metadata: &Metadata,
	previous_index: &BuildIndex,
	current_index: &mut BuildIndex,
	index_file: &Path,
	total_modules: &mut usize,
	total_units: &mut usize,
	successful_modules: &mut usize,
	successful_units: &mut usize,
) -> Result<(), LoadModuleError> {
	let error = || LoadModuleError {
		dir: module_dir.to_path_buf(),
	};

	*total_modules += 1;
	let module = module_dir.join("module.toml");
	let module = load_manifest::<Module>(&module).change_context_lazy(error)?;
	let metadata = parent_metadata
		.iter()
		.chain(module.metadata.iter())
		.map(|(key, value)| (key.clone(), value.clone()))
		.collect::<Metadata>();
	*successful_modules += 1;

	match &module.children {
		Children::Modules { modules } => {
			for child_dir in modules {
				let child_dir = module_dir.join(child_dir);
				if let Err(e) = recurse_modules(
					&child_dir,
					&metadata,
					previous_index,
					current_index,
					index_file,
					total_modules,
					total_units,
					successful_modules,
					successful_units,
				) {
					eprintln!("{e:?}");
				}
			}
		}

		Children::Files { files } => {
			if let Err(e) = incremental_build(
				module_dir,
				&metadata,
				files,
				previous_index,
				current_index,
				index_file,
				total_units,
				successful_units,
			) {
				eprintln!("{e:?}");
			};
		}
	}

	Ok(())
}

#[derive(Debug, Error)]
#[error("Could not load module {dir:?}")]
pub struct LoadModuleError {
	dir: PathBuf,
}

#[derive(Debug, Error)]
pub enum PrepareBuildError {
	#[error("Failed to build source directory {dir:?}")]
	Build { dir: PathBuf },

	#[error("Failed to check source directory {dir:?}")]
	Check { dir: PathBuf },
}
