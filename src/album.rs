// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::filesystem::require_dir;
use crate::manifest::load_manifest;
use crate::result::Result;

#[derive(Debug)]
pub struct Album {
	path: PathBuf,
	manifest: AlbumManifest,
}

impl Album {
	pub fn load(path: &Path) -> Result<Self, AlbumError> {
		let error = || AlbumError::Load {
			path: path.to_path_buf(),
		};

		require_dir(path).change_context_lazy(error)?;

		let manifest = path.join("album.toml");
		let manifest = load_manifest(&manifest).change_context_lazy(error)?;

		Ok(Album {
			path: path.to_path_buf(),
			manifest,
		})
	}

	pub fn path(&self) -> &Path {
		&self.path
	}
}

impl PartialEq for Album {
	fn eq(&self, other: &Self) -> bool {
		self.manifest.id == other.manifest.id
	}
}

impl Eq for Album {}

impl Hash for Album {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.manifest.id.hash(state);
	}
}

impl Display for Album {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let AlbumIdentifier { title, artist, year } = &self.manifest.id;
		write!(f, "{artist} - ({year}) {title}")
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
