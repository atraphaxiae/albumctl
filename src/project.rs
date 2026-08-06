// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::album::Album;
use crate::filesystem::{ensure_dir, list_dirs, require_absent, require_dir};
use crate::manifest::{load_manifest, save_manifest};
use crate::result::Result;

#[derive(Debug)]
pub struct Project {
	root: PathBuf,
	manifest: ProjectManifest,
}

impl Project {
	pub fn init(path: &Path) -> Result<Self, ProjectError> {
		let error = || ProjectError::Init {
			path: path.to_path_buf(),
		};

		ensure_dir(path).change_context_lazy(error)?;

		let manifest_path = path.join("albumctl.toml");
		require_absent(&manifest_path).change_context_lazy(error)?;

		let manifest = ProjectManifest {
			output_dir: "~/Music".into(),
		};

		save_manifest(&manifest_path, &manifest).change_context_lazy(error)?;

		Ok(Project {
			root: path.to_path_buf(),
			manifest,
		})
	}

	pub fn load(path: &Path) -> Result<Self, ProjectError> {
		let error = || ProjectError::Load {
			path: path.to_path_buf(),
		};

		require_dir(path).change_context_lazy(error)?;

		let manifest = path.join("albumctl.toml");
		let manifest = load_manifest(&manifest).change_context_lazy(error)?;

		Ok(Project {
			root: path.to_path_buf(),
			manifest,
		})
	}

	pub fn check(&self) -> Result<(), ProjectError> {
		let error = || ProjectError::Check {
			path: self.root.clone(),
		};

		let albums = self.load_albums().change_context(error())?;
		for album in albums {
			album.load_releases().change_context(error())?;
		}

		Ok(())
	}

	pub fn load_albums(&self) -> Result<HashSet<Album>, ProjectError> {
		let error = || ProjectError::LoadAlbums {
			path: self.root.clone(),
		};

		let mut albums = HashSet::<Album>::new();
		for dir in list_dirs(&self.root).change_context_lazy(error)? {
			let album = Album::load(&dir).change_context_lazy(error)?;
			if let Some(original) = albums.get(&album) {
				Err(error()).attach(formatdoc!(
					r#"
						duplicate albums of "{album}" found at:
							- {}
							- {}
					"#,
					album.path().display(),
					original.path().display()
				))?;
			}
			albums.insert(album);
		}

		Ok(albums)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectManifest {
	pub output_dir: PathBuf,
}

#[derive(Debug, Error)]
pub enum ProjectError {
	#[error("Could not initialize albumctl project at {path:?}")]
	Init { path: PathBuf },

	#[error("Could not load albumctl project at {path:?}")]
	Load { path: PathBuf },

	#[error("Detected errors in project at {path:?}")]
	Check { path: PathBuf },

	#[error("Could not load albums of project at {path:?}")]
	LoadAlbums { path: PathBuf },
}
