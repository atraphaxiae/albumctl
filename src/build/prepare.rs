// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	collections::HashMap,
	ffi::OsString,
	path::{Components, Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::incremental::incremental_build,
	filesystem::{ensure_dir, ensure_file},
	manifest::load_manifest,
	module::{Children, Disc, Metadata, Module, RootModule},
	result::Result,
};

pub type BuildIndex = HashMap<String, PathBuf>;

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
	let mut successful_modules = 0_usize;

	let mut total_units = 0_usize;
	let mut built_units = 0_usize;
	let mut skipped_units = 0_usize;

	for child_dir in root_module.children.modules {
		let child_dir = dir.join(child_dir);
		if let Err(e) = recurse_modules(
			&child_dir,
			&root_module.metadata,
			None,
			&previous_index,
			&mut current_index,
			&index_file,
			&root_module.output_directory,
			&mut total_modules,
			&mut successful_modules,
			&mut total_units,
			&mut built_units,
			&mut skipped_units,
		) {
			eprintln!("{e:?}");
		}
	}

	println!("Building source directory {dir:?} completed.");
	println!("Output: {:?}", &root_module.output_directory);
	println!("{successful_modules}/{total_modules} discovered modules loaded successfully.");
	println!(
		"{} built, {} skipped, {} failed, out of {} discovered units.",
		built_units,
		skipped_units,
		total_units - (built_units + skipped_units),
		total_units,
	);

	Ok(())
}

pub fn check(dir: &Path) -> Result<(), PrepareBuildError> {
	todo!()
}

fn recurse_modules(
	module_dir: &Path,
	parent_metadata: &Metadata,
	parent_tracklist: Option<&[Disc]>,
	previous_index: &BuildIndex,
	current_index: &mut BuildIndex,
	index_file: &Path,
	output_dir: &Path,
	total_modules: &mut usize,
	successful_modules: &mut usize,
	total_units: &mut usize,
	built_units: &mut usize,
	skipped_units: &mut usize,
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
	let tracklist = module.discs.as_deref().or(parent_tracklist);

	match module.children {
		Children::Modules { modules } => {
			*successful_modules += 1;
			for child_dir in modules {
				let child_dir = module_dir.join(child_dir);
				if let Err(e) = recurse_modules(
					&child_dir,
					&metadata,
					tracklist,
					previous_index,
					current_index,
					index_file,
					output_dir,
					total_modules,
					successful_modules,
					total_units,
					built_units,
					skipped_units,
				) {
					eprintln!("{e:?}");
				}
			}
		}

		Children::Files { mut files } => {
			let Some(tracklist) = tracklist else {
				return Err(error()).attach(
					"a tracklist is required to be defined in at least one module in this lineage for this unit to be built",
				);
			};

			files.sort_by_key(|file| (file.disc_number, file.track_number));
			let expected = tracklist
				.iter()
				.enumerate()
				.flat_map(|(disc_number, disc)| {
					disc.tracks
						.iter()
						.enumerate()
						.map(move |(track_number, _)| (disc_number + 1, track_number + 1))
				});
			let actual = files
				.iter()
				.map(|file| (file.disc_number, file.track_number));
			if actual.ne(expected) {
				return Err(error()).attach("source file mapping doesn't match the tracklist");
			}

			*successful_modules += 1;
			if let Err(e) = incremental_build(
				module_dir,
				&metadata,
				tracklist,
				&files,
				previous_index,
				current_index,
				index_file,
				output_dir,
				total_units,
				built_units,
				skipped_units,
			) {
				eprintln!("{e:?}");
			};
		}
	}

	Ok(())
}

#[derive(Debug)]
struct FsTree {
	root: FsTreeNode,
}

impl FsTree {
	fn new() -> Self {
		Self {
			root: FsTreeNode::new(),
		}
	}

	fn insert(&mut self, dir: &Path) {
		let mut components = dir.components();
		self.root.insert(&mut components);
	}

	fn traverse(&self, dir: &Path) -> Option<&FsTreeNode> {
		let mut components = dir.components();
		self.root.traverse(&mut components)
	}
}

#[derive(Debug)]
struct FsTreeNode {
	children: HashMap<OsString, FsTreeNode>,
}

impl FsTreeNode {
	fn new() -> Self {
		Self {
			children: HashMap::new(),
		}
	}

	fn insert(&mut self, components: &mut Components) {
		let Some(component) = components.next() else {
			return;
		};

		let component = component.as_os_str().to_os_string();
		let next = self
			.children
			.entry(component)
			.or_insert_with(FsTreeNode::new);

		next.insert(components);
	}

	fn traverse(&self, components: &mut Components) -> Option<&Self> {
		let Some(component) = components.next() else {
			return Some(self);
		};

		let component = component.as_os_str();
		let next = self.children.get(component)?;

		next.traverse(components)
	}
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
