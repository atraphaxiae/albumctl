// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use thiserror::Error;

pub struct SourceDir {
	dir: PathBuf,
	config_file: PathBuf,
	albums: Vec<AlbumDir>,
}

pub struct AlbumDir {
	dir: PathBuf,
	manifest_file: PathBuf,
	releases: Vec<ReleaseDir>,
}

pub struct ReleaseDir {
	dir: PathBuf,
	manifest_file: PathBuf,
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
