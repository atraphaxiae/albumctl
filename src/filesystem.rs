// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	fs::{File, OpenOptions},
	io,
	path::{Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;

pub fn list_dirs(dir: &Path) -> Result<Vec<PathBuf>, FilesystemError> {
	let error = || FilesystemError::ListDirs {
		dir: dir.to_path_buf(),
	};

	let entries = dir
		.read_dir()
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {dir:?}"))?
		.collect::<io::Result<Vec<_>>>()
		.change_context_lazy(error)
		.attach_with(|| format!("while enumerating entries of {dir:?}"))?;

	let dirs = entries
		.into_iter()
		.filter_map(|entry| {
			let path = entry.path();
			match entry.file_type() {
				Ok(file_type) => file_type.is_dir().then_some(Ok(path)),
				Err(e) => Some(
					Err(e)
						.change_context(error())
						.attach(format!("while reading file type of {path:?}")),
				),
			}
		})
		.collect::<Result<Vec<_>, _>>()?;

	Ok(dirs)
}

pub fn copy_file(from: &Path, to: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::CopyFile {
		from: from.to_path_buf(),
		to: to.to_path_buf(),
	};

	let mut reader = File::open(from)
		.change_context_lazy(error)
		.attach_with(|| format!("while opening {from:?}"))?;

	let mut writer = OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(to)
		.change_context_lazy(error)
		.attach_with(|| format!("while opening {to:?}"))?;

	io::copy(&mut reader, &mut writer)
		.change_context_lazy(error)
		.attach_with(|| format!("while copying from {from:?} to {to:?}"));

	Ok(())
}

#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("Could not list the immediate child directories of {dir:?}")]
	ListDirs { dir: PathBuf },

	#[error("Could not copy file from {from:?} to {to:?}")]
	CopyFile { from: PathBuf, to: PathBuf },
}
