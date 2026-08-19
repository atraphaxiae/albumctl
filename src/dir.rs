// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

//! Module for resolving the source directory structure.
//!
//! We are only concerned with resolving the structure of the source directory in this layer. No
//! validation or file opening is done here.
//!
//! Given a source directory that looks like this:
//! ```text
//! /src
//! ├── albumctl.toml
//! └── Speak No Evil
//!     ├── album.toml
//!     └── MM33 Vinyl Rip
//!         ├── release.toml
//!         ├── 01 - Witch Hunt.flac
//!         └── ...
//! ```
//!
//! `SourceDir::resolve(path)` where `path` is `/src` will resolve it into this:
//! ```text
//! SourceDir
//! ├── dir: /src
//! ├── config_file: /src/albumctl.toml
//! └── albums:
//!     └── AlbumDir
//!         ├── dir: /src/Speak No Evil
//!         ├── manifest_file: /src/Speak No Evil/album.toml
//!         └── releases:
//!             └── ReleaseDir
//!                 ├── dir: /src/Speak No Evil/MM33 Vinyl Rip
//!                 └── manifest_file: /src/Speak No Evil/MM33 Vinyl Rip/release.toml
//! ```

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{filesystem::list_dirs, result::Result};

#[derive(Debug)]
pub struct SourceDir {
	pub dir: PathBuf,
	pub config_file: PathBuf,
	pub albums: Vec<AlbumDir>,
}

impl SourceDir {
	pub fn resolve(dir: &Path) -> Result<Self, DirError> {
		let error = || DirError::SourceResolve {
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

#[derive(Debug)]
pub struct AlbumDir {
	pub dir: PathBuf,
	pub manifest_file: PathBuf,
	pub releases: Vec<ReleaseDir>,
}

impl AlbumDir {
	pub fn resolve(dir: &Path) -> Result<Self, DirError> {
		let error = || DirError::AlbumResolve {
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

#[derive(Debug)]
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
	#[error("Could not resolve the source directory {dir:?}")]
	SourceResolve { dir: PathBuf },

	#[error("Could not resolve the album directory {dir:?}")]
	AlbumResolve { dir: PathBuf },
}
