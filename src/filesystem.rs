// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;

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

pub fn require_dir(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::RequireDir {
		path: path.to_path_buf(),
	};

	match path.metadata() {
		Ok(metadata) if metadata.is_dir() => Ok(()),
		Ok(_) => Err(error()).attach(format!("{path:?} is not a directory")),
		Err(e) if e.kind() == ErrorKind::NotFound => {
			Err(error()).attach(format!("{path:?} does not exist"))
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while reading metadata of {path:?}")),
	}
}

pub fn require_file(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::RequireFile {
		path: path.to_path_buf(),
	};

	match path.metadata() {
		Ok(metadata) if metadata.is_file() => Ok(()),
		Ok(_) => Err(error()).attach(format!("{path:?} is not a file")),
		Err(e) if e.kind() == ErrorKind::NotFound => {
			Err(error()).attach(format!("{path:?} does not exist"))
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while reading metadata of {path:?}")),
	}
}

pub fn ensure_dir(path: &Path) -> Result<(), FilesystemError> {
	create_dir_all(path)
		.change_context_lazy(|| FilesystemError::EnsureDir {
			path: path.to_path_buf(),
		})
		.attach_with(|| format!("while creating {path:?}"))
}

pub fn ensure_file(path: &Path, content: Option<&str>) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureFile {
		path: path.to_path_buf(),
	};

	match OpenOptions::new().write(true).create_new(true).open(path) {
		Ok(mut file) if let Some(content) = content => file
			.write_all(content.as_bytes())
			.change_context_lazy(error)
			.attach_with(|| format!("while writing to {path:?}")),
		Ok(_) => Ok(()),
		Err(e) if e.kind() == ErrorKind::AlreadyExists => {
			require_file(path).change_context_lazy(error)
		}
		Err(e) => Err(e)
			.change_context(error())
			.attach(format!("while opening {path:?}")),
	}
}

pub fn list_dirs(path: &Path) -> Result<Vec<PathBuf>, FilesystemError> {
	let error = || FilesystemError::ListDirs {
		path: path.to_path_buf(),
	};

	let entries = path
		.read_dir()
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {path:?}"))?
		.collect::<io::Result<Vec<_>>>()
		.change_context_lazy(error)
		.attach_with(|| format!("while enumerating entries of {path:?}"))?;

	let dirs = entries
		.into_iter()
		.filter_map(|entry| {
			let path = entry.path();
			match entry.metadata() {
				Ok(metadata) => metadata.is_dir().then_some(Ok(path)),
				Err(e) => Some(
					Err(e)
						.change_context_lazy(error)
						.attach_with(|| format!("while reading metadata of {path:?}")),
				),
			}
		})
		.collect::<Result<Vec<_>, _>>()?;

	Ok(dirs)
}

#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("Expected no file or directory at {path:?}")]
	RequireAbsent { path: PathBuf },

	#[error("Expected a directory at {path:?}")]
	RequireDir { path: PathBuf },

	#[error("Expected a file at {path:?}")]
	RequireFile { path: PathBuf },

	#[error("Could not ensure a directory exists at {path:?}")]
	EnsureDir { path: PathBuf },

	#[error("Could not ensure a file exists at {path:?}")]
	EnsureFile { path: PathBuf },

	#[error("Could not list subdirectories of {path:?}")]
	ListDirs { path: PathBuf },
}
