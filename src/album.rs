// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::manifest::load_manifest;
use crate::result::Result;

#[derive(Debug)]
pub struct Album {
	path: PathBuf,
	manifest: AlbumManifest,
}

impl Album {
	pub fn load(path: &Path) -> Result<Self, AlbumError> {
		let manifest = path.join("album.toml");
		let manifest = load_manifest(&manifest).change_context_lazy(|| AlbumError::Load {
			path: path.to_path_buf(),
		})?;

		Ok(Album {
			path: path.to_path_buf(),
			manifest,
		})
	}
}

#[derive(Debug, Deserialize)]
pub struct AlbumManifest {
	#[serde(flatten)]
	pub id: AlbumIdentifier,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct AlbumIdentifier {
	pub title: String,
	pub artist: String,
	pub year: u16,
}

#[derive(Debug, Error)]
pub enum AlbumError {
	#[error("Could not load album at {path:?}")]
	Load { path: PathBuf },
}
