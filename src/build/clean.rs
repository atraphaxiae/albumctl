// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	collections::HashMap,
	ffi::OsString,
	fs::{DirEntry, read_dir},
	io,
	path::{Components, Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::BuildIndex,
	filesystem::{delete_item, get_item_type},
	result::Result,
};

pub fn clean_output_dir(
	current_index: &BuildIndex,
	output_dir: &Path,
) -> Result<usize, CleanError> {
	let error = || CleanError::Prepare {
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
) -> Result<(), CleanError> {
	let error = || CleanError::Dir {
		dir: dir.to_path_buf(),
	};

	for entry in read_dir(dir)
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {dir:?}"))?
	{
		match clean_item(entry, dir, node, stack, output_dir) {
			Ok(true) => *deleted_items += 1,
			Ok(false) => (),
			Err(e) => eprintln!("{e:?}\n"),
		}
	}

	Ok(())
}

fn clean_item<'a>(
	entry: std::result::Result<DirEntry, io::Error>,
	dir: &Path,
	node: &'a FsTreeNode,
	stack: &mut Vec<(PathBuf, &'a FsTreeNode)>,
	output_dir: &Path,
) -> Result<bool, CleanError> {
	let error = || CleanError::Item {
		dir: dir.to_path_buf(),
	};

	let entry = entry
		.change_context_lazy(error)
		.attach_with(|| format!("while reading an entry of {dir:?}"))?;

	if dir == output_dir && entry.file_name() == ".albumctl" {
		return Ok(false);
	}

	let item = entry.path();
	if let Some(child) = node.children.get(&entry.file_name()) {
		if get_item_type(&item).change_context_lazy(error)?.is_dir() {
			stack.push((item, child));
		}

		Ok(false)
	} else {
		delete_item(&item)
			.change_context_lazy(error)
			.attach_with(|| format!("while deleting {item:?}"))?;

		Ok(true)
	}
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
pub enum CleanError {
	#[error("Could not prepare to clean output directory {dir:?}")]
	Prepare { dir: PathBuf },

	#[error("Could not clean directory {dir:?}")]
	Dir { dir: PathBuf },

	#[error("Could not clean item in {dir:?}")]
	Item { dir: PathBuf },
}
