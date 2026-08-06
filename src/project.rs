// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::album::Album;
use crate::filesystem::{ensure_dir, list_dirs, require_absent};
use crate::manifest::save_manifest;
use crate::result::Result;

#[derive(Debug)]
pub struct Project {
	root: PathBuf,
}

impl Project {
	pub fn init(path: &Path) -> Result<Self, ProjectError> {
		let error = || ProjectError::Init {
			path: path.to_path_buf(),
		};

		ensure_dir(path).change_context_lazy(error)?;

		let manifest = path.join("albumctl.toml");
		require_absent(&manifest).change_context_lazy(error)?;

		save_manifest(
			&manifest,
			&ProjectManifest {
				output_dir: "~/Music".into(),
			},
		)
		.change_context_lazy(error)?;

		Ok(Project {
			root: path.to_path_buf(),
		})
	}

	pub fn load_albums(&self) -> Result<Vec<Album>, ProjectError> {
		let error = || ProjectError::LoadAlbums {
			path: self.root.clone(),
		};

		list_dirs(&self.root)
			.change_context_lazy(error)?
			.into_iter()
			.map(|dir| Album::load(&dir))
			.collect::<Result<Vec<_>, _>>()
			.change_context_lazy(error)
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

	#[error("Could not load albums of project at {path:?}")]
	LoadAlbums { path: PathBuf },
}
