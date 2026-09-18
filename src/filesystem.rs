// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	fs::{FileType, OpenOptions, copy, create_dir_all, remove_dir_all, remove_file, rename},
	io::{ErrorKind, Write},
	path::{Path, PathBuf},
	time::SystemTime,
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;

pub fn get_item_type(item: &Path) -> Result<FileType, FilesystemError> {
	let error = || FilesystemError::GetItemType {
		item: item.to_path_buf(),
	};

	match item.metadata() {
		Ok(metadata) => Ok(metadata.file_type()),
		Err(e) if e.kind() == ErrorKind::NotFound => {
			Err(error()).attach(format!("{item:?} does not exist"))
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while reading metadata of {item:?}")),
	}
}

pub fn get_mtime_size(item: &Path) -> Result<(SystemTime, u64), FilesystemError> {
	let error = || FilesystemError::GetMtimeSize {
		item: item.to_path_buf(),
	};

	match item.metadata() {
		Ok(metadata) => Ok((
			metadata
				.modified()
				.change_context_lazy(error)
				.attach_with(|| format!("while getting mtime of {item:?}"))?,
			metadata.len(),
		)),
		Err(e) if e.kind() == ErrorKind::NotFound => {
			Err(error()).attach(format!("{item:?} does not exist"))
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while reading metadata of {item:?}")),
	}
}

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

pub fn ensure_file(file: &Path, content: Option<&str>) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureFile {
		file: file.to_path_buf(),
	};

	match OpenOptions::new().write(true).create_new(true).open(file) {
		Ok(mut file) if let Some(content) = content => file
			.write_all(content.as_bytes())
			.change_context_lazy(error)
			.attach_with(|| format!("while writing to {file:?}")),
		Ok(_) => Ok(()),
		Err(e) if e.kind() == ErrorKind::AlreadyExists => {
			require_file(file).change_context_lazy(error)
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while opening {file:?}")),
	}
}

pub fn ensure_dir(dir: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureDir {
		dir: dir.to_path_buf(),
	};

	create_dir_all(dir)
		.change_context_lazy(error)
		.attach_with(|| format!("while creating {dir:?} and all of its parents"))?;

	Ok(())
}

pub fn delete_item(item: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::Delete {
		item: item.to_path_buf(),
	};

	let metadata = match item.metadata() {
		Ok(metadata) => metadata,
		Err(e) if e.kind() == ErrorKind::NotFound => return Ok(()),
		Err(e) => {
			return Err(e)
				.change_context(error())
				.attach("while reading metadata of path");
		}
	};

	if metadata.is_file() {
		remove_file(item)
			.change_context_lazy(error)
			.attach_with(|| format!("while deleting {item:?}"))?;
	} else if metadata.is_dir() {
		remove_dir_all(item)
			.change_context_lazy(error)
			.attach_with(|| format!("while deleting {item:?}"))?;
	}

	Ok(())
}

pub fn copy_file(from: &Path, to: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::CopyFile {
		from: from.to_path_buf(),
		to: to.to_path_buf(),
	};

	if let Some(to_dir) = to.parent() {
		ensure_dir(to_dir).change_context_lazy(error)?;
	}

	copy(from, to).change_context_lazy(error)?;

	Ok(())
}

pub fn move_file(from: &Path, to: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::MoveFile {
		from: from.to_path_buf(),
		to: to.to_path_buf(),
	};

	if let Some(to_dir) = to.parent() {
		ensure_dir(to_dir).change_context_lazy(error)?;
	}

	rename(from, to).change_context_lazy(error)?;

	Ok(())
}

#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("Could not get item type of {item:?}")]
	GetItemType { item: PathBuf },

	#[error("Could not get mtime and size of {item:?}")]
	GetMtimeSize { item: PathBuf },

	#[error("Expected a file at {file:?}")]
	RequireFile { file: PathBuf },

	#[error("Could not ensure file exists at {file:?}")]
	EnsureFile { file: PathBuf },

	#[error("Could not ensure directory exists at {dir:?}")]
	EnsureDir { dir: PathBuf },

	#[error("Could not delete {item:?}")]
	Delete { item: PathBuf },

	#[error("Could not copy file from {from:?} to {to:?}")]
	CopyFile { from: PathBuf, to: PathBuf },

	#[error("Could not move file from {from:?} to {to:?}")]
	MoveFile { from: PathBuf, to: PathBuf },
}
