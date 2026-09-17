// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	collections::HashMap,
	ffi::OsString,
	fs::{DirEntry, read_dir},
	io,
	path::{Components, Path, PathBuf},
};

use colored::Colorize;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::incremental::incremental_build,
	filesystem::{delete, ensure_dir, ensure_file, get_file_type},
	manifest::load_manifest,
	module::{Children, Disc, File, Metadata, Module, RootModule},
	result::Result,
};

pub type BuildIndex = HashMap<String, Vec<PathBuf>>;

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
			eprintln!("{e:?}\n");
		}
	}

	let clean_result = clean_output_dir(&current_index, &root_module.output_directory);
	if let Err(e) = &clean_result {
		eprintln!("{e:?}\n");
	}

	println!("{}", "Build completed.".green().bold());
	println!("├╴Source: {}", dir.display().to_string().cyan());
	println!(
		"├╴Output: {}",
		root_module.output_directory.display().to_string().cyan()
	);
	println!("│");
	println!(
		"├╴Modules: {}",
		format!("{}/{} loaded", successful_modules, total_modules).green()
	);
	println!(
		"├╴Units:   {}, {}, {}",
		format!("{} built", built_units).green(),
		format!("{} skipped", skipped_units).cyan(),
		format!("{} failed", total_units - (built_units + skipped_units)).red(),
	);

	if let Ok(deleted_items) = clean_result {
		println!(
			"╰╴Cleanup: {}",
			format!("{} unmanaged items deleted", deleted_items).green()
		);
	} else {
		println!("╰╴Cleanup: {}", "failed".red());
	}

	Ok(())
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
					eprintln!("{e:?}\n");
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

			let mut files_map = HashMap::<&Path, &File>::new();
			for file in &files {
				if let Some(duplicate) = files_map.insert(&file.file, &file) {
					return Err(error()).attach(format!(
						"Tracks {}.{:02} and {}.{:02} are mapped to the same source file {:?}",
						duplicate.disc_number,
						duplicate.track_number,
						file.disc_number,
						file.track_number,
						file.file
					));
				}
			}

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
				eprintln!("{e:?}\n");
			};
		}
	}

	Ok(())
}

fn clean_output_dir(
	current_index: &BuildIndex,
	output_dir: &Path,
) -> Result<usize, CleanOutputDirError> {
	let error = || CleanOutputDirError::Prepare {
		dir: output_dir.to_path_buf(),
	};

	let mut fs_tree = FsTree::new();
	for files in current_index.values() {
		for file in files {
			fs_tree.insert(file);
		}
	}

	let output_fs_tree = fs_tree.traverse(output_dir).ok_or_else(error)?;
	let mut stack = vec![(output_dir.to_path_buf(), output_fs_tree)];
	let mut deleted_items = 0;

	while let Some((dir, node)) = stack.pop() {
		if let Err(e) = clean_dir(&dir, node, &mut stack, &mut deleted_items, output_dir) {
			eprintln!("{e:?}\n");
		}
	}

	Ok(deleted_items)
}

fn clean_dir<'a>(
	dir: &Path,
	node: &'a FsTreeNode,
	stack: &mut Vec<(PathBuf, &'a FsTreeNode)>,
	deleted_items: &mut usize,
	output_dir: &Path,
) -> Result<(), CleanOutputDirError> {
	let error = || CleanOutputDirError::CleanDir {
		dir: dir.to_path_buf(),
	};

	for entry in read_dir(dir)
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {dir:?}"))?
	{
		if let Err(e) = clean_entries(entry, dir, node, stack, deleted_items, output_dir) {
			eprintln!("{e:?}\n");
		}
	}

	Ok(())
}

fn clean_entries<'a>(
	entry: std::result::Result<DirEntry, io::Error>,
	dir: &Path,
	node: &'a FsTreeNode,
	stack: &mut Vec<(PathBuf, &'a FsTreeNode)>,
	deleted_items: &mut usize,
	output_dir: &Path,
) -> Result<(), CleanOutputDirError> {
	let error = || CleanOutputDirError::CleanDir {
		dir: dir.to_path_buf(),
	};

	let entry = entry
		.change_context_lazy(error)
		.attach_with(|| format!("while reading entries of {dir:?}"))?;

	let path = entry.path();

	if dir == output_dir && entry.file_name() == ".albumctl" {
		return Ok(());
	}

	if let Some(child) = node.children.get(&entry.file_name()) {
		if get_file_type(&path).change_context_lazy(error)?.is_dir() {
			stack.push((path, child));
		}
	} else {
		delete(&path)
			.change_context_lazy(error)
			.attach_with(|| format!("while deleting {path:?}"))?;
		*deleted_items += 1;
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

	fn insert(&mut self, path: &Path) {
		let mut components = path.components();
		self.root.insert(&mut components);
	}

	fn traverse(&self, path: &Path) -> Option<&FsTreeNode> {
		let mut components = path.components();
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
pub enum CleanOutputDirError {
	#[error("Could not prepare to clean output directory {dir:?}")]
	Prepare { dir: PathBuf },

	#[error("Could not clean directory {dir:?}")]
	CleanDir { dir: PathBuf },
}

#[derive(Debug, Error)]
pub enum PrepareBuildError {
	#[error("Failed to build source directory {dir:?}")]
	Build { dir: PathBuf },
}
