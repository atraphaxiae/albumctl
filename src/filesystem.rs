// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	fs::{File, OpenOptions, create_dir_all, remove_dir_all, rename},
	io::{self, ErrorKind},
	path::{Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;

/// Errors if there is either a directory or a file at `path`.
pub fn require_absent(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::RequireAbsent {
		path: path.to_path_buf(),
	};

	match path.try_exists() {
		Ok(false) => Ok(()),
		Ok(true) => Err(error()).attach(format!("{path:?} exists")),
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while checking if {path:?} exists")),
	}
}

/// Errors if `file` is not a file.
pub fn require_file(file: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::RequireFile {
		file: file.to_path_buf(),
	};

	match file.metadata() {
		Ok(metadata) if metadata.is_file() => Ok(()),
		Ok(_) => Err(error()).attach(format!("{file:?} is not a file")),
		Err(e) if e.kind() == ErrorKind::NotFound => {
			Err(error()).attach(format!("{file:?} does not exist"))
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while reading metadata of {file:?}")),
	}
}

/// Creates `dir` and all its missing parents. Does not error if `dir` already exists.
pub fn ensure_dir(dir: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureDir {
		dir: dir.to_path_buf(),
	};

	create_dir_all(dir)
		.change_context_lazy(error)
		.attach_with(|| format!("while creating {dir:?}"))?;

	Ok(())
}

/// Deletes `dir` and all of its contents. Does not error if `dir` doesn't exist.
pub fn delete_dir(dir: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::DeleteDir {
		dir: dir.to_path_buf(),
	};

	match remove_dir_all(dir) {
		Ok(()) => Ok(()),
		Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while deleting {dir:?}")),
	}
}

/// Lists immediate child directories of `dir`.
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

/// Moves a file from `from` to `to`. Does not work if `from` and `to` are on different filesystems.
/// This errors if `to` already exists, however this is a TOCTOU case and is not guaranteed.
pub fn move_file(from: &Path, to: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::MoveFile {
		from: from.to_path_buf(),
		to: to.to_path_buf(),
	};

	require_absent(to).change_context_lazy(error)?;
	rename(from, to)
		.change_context_lazy(error)
		.attach_with(|| format!("while moving from {from:?} to {to:?}"))?;

	Ok(())
}

/// Copies a file from `from` to `to`. This will error if `to` already exists.
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
		.attach_with(|| format!("while copying from {from:?} to {to:?}"))?;

	Ok(())
}

#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("Expected no file or directory at {path:?}")]
	RequireAbsent { path: PathBuf },

	#[error("Expected a file at {file:?}")]
	RequireFile { file: PathBuf },

	#[error("Could not ensure directory exists at {dir:?}")]
	EnsureDir { dir: PathBuf },

	#[error("Could not delete directory at {dir:?}")]
	DeleteDir { dir: PathBuf },

	#[error("Could not list the immediate child directories of {dir:?}")]
	ListDirs { dir: PathBuf },

	#[error("Could not move file from {from:?} to {to:?}")]
	MoveFile { from: PathBuf, to: PathBuf },

	#[error("Could not copy file from {from:?} to {to:?}")]
	CopyFile { from: PathBuf, to: PathBuf },
}
