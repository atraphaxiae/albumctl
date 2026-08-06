// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use indoc::formatdoc;
use serde::Deserialize;
use thiserror::Error;

use crate::filesystem::{list_dirs, require_dir};
use crate::manifest::load_manifest;
use crate::release::Release;
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

	pub fn load_releases(&self) -> Result<HashSet<Release>, AlbumError> {
		let error = || AlbumError::LoadReleases {
			id: self.manifest.id.clone(),
		};

		let mut releases = HashSet::<Release>::new();
		for dir in list_dirs(&self.path).change_context_lazy(error)? {
			let release = Release::load(&dir).change_context_lazy(error)?;
			if let Some(original) = releases.get(&release) {
				Err(error()).attach(formatdoc!(
					r#"
						duplicate releases of "{release}" found at:
							- {}
							- {}
					"#,
					release.path().display(),
					original.path().display()
				))?;
			}
			releases.insert(release);
		}

		Ok(releases)
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
		self.manifest.id.fmt(f)
	}
}

#[derive(Debug, Deserialize)]
pub struct AlbumManifest {
	#[serde(flatten)]
	pub id: AlbumIdentifier,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct AlbumIdentifier {
	pub title: String,
	pub artist: String,
	pub year: u16,
}

impl Display for AlbumIdentifier {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let Self {
			artist,
			year,
			title,
		} = self;
		write!(f, "{artist} - ({year}) {title}")
	}
}

#[derive(Debug, Error)]
pub enum AlbumError {
	#[error("Could not load album at {path:?}")]
	Load { path: PathBuf },

	#[error("Could not load releases of album \"{id}\"")]
	LoadReleases { id: AlbumIdentifier },
}
