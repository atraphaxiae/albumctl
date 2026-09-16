// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	fs::{OpenOptions, create_dir_all},
	io::{ErrorKind, Write},
	path::{Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;

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

#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("Expected a file at {file:?}")]
	RequireFile { file: PathBuf },

	#[error("Could not ensure file exists at {file:?}")]
	EnsureFile { file: PathBuf },

	#[error("Could not ensure directory exists at {dir:?}")]
	EnsureDir { dir: PathBuf },
}
