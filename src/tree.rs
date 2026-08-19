// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use thiserror::Error;

pub struct SourceTree {
	dir: PathBuf,
	config_file: PathBuf,
	albums: Vec<AlbumTree>,
}

pub struct AlbumTree {
	dir: PathBuf,
	manifest_file: PathBuf,
	releases: Vec<ReleaseTree>,
}

pub struct ReleaseTree {
	dir: PathBuf,
	manifest_file: PathBuf,
}

impl ReleaseTree {
	pub fn from_dir(dir: &Path) -> Self {
		Self {
			dir: dir.to_path_buf(),
			manifest_file: dir.join("release.toml")
		}
	}
}

#[derive(Debug, Error)]
pub enum TreeError {
	#[error("Could not create source tree from directory {dir:?}")]
	SourceTreeFrom { dir: PathBuf },

	#[error("Could not create album tree from directory {dir:?}")]
	AlbumTreeFrom { dir: PathBuf }
}
