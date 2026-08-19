// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{filesystem::list_dirs, result::Result};

pub struct SourceDir {
	pub dir: PathBuf,
	pub config_file: PathBuf,
	pub albums: Vec<AlbumDir>,
}

impl SourceDir {
	pub fn resolve(dir: &Path) -> Result<Self, DirError> {
		let error = || DirError::SourceDirResolve {
			dir: dir.to_path_buf(),
		};

		Ok(Self {
			dir: dir.to_path_buf(),
			config_file: dir.join("albumctl.toml"),
			albums: list_dirs(dir)
				.change_context_lazy(error)?
				.into_iter()
				.map(|album_dir| AlbumDir::resolve(&album_dir))
				.collect::<Result<_, _>>()?,
		})
	}
}

pub struct AlbumDir {
	pub dir: PathBuf,
	pub manifest_file: PathBuf,
	pub releases: Vec<ReleaseDir>,
}

impl AlbumDir {
	pub fn resolve(dir: &Path) -> Result<Self, DirError> {
		let error = || DirError::AlbumDirResolve {
			dir: dir.to_path_buf(),
		};

		Ok(Self {
			dir: dir.to_path_buf(),
			manifest_file: dir.join("album.toml"),
			releases: list_dirs(dir)
				.change_context_lazy(error)?
				.into_iter()
				.map(|release_dir| ReleaseDir::resolve(&release_dir))
				.collect(),
		})
	}
}

pub struct ReleaseDir {
	pub dir: PathBuf,
	pub manifest_file: PathBuf,
}

impl ReleaseDir {
	pub fn resolve(dir: &Path) -> Self {
		Self {
			dir: dir.to_path_buf(),
			manifest_file: dir.join("release.toml"),
		}
	}
}

#[derive(Debug, Error)]
pub enum DirError {
	#[error("Could not resolve {dir:?} as a source directory")]
	SourceDirResolve { dir: PathBuf },

	#[error("Could not resolve {dir:?} as an album directory")]
	AlbumDirResolve { dir: PathBuf },
}
