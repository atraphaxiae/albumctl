// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::filesystem::require_dir;
use crate::manifest::load_manifest;
use crate::mapping::MappingManifest;
use crate::result::Result;
use crate::tracklist::Disc;

#[derive(Debug)]
pub struct Release {
	path: PathBuf,
	manifest: ReleaseManifest,
	mapping: MappingManifest,
}

impl Release {
	pub fn load(path: &Path) -> Result<Self, ReleaseError> {
		let error = || ReleaseError::Load {
			path: path.to_path_buf(),
		};

		require_dir(path).change_context_lazy(error)?;

		let manifest = path.join("release.toml");
		let manifest = load_manifest(&manifest).change_context_lazy(error)?;

		let mapping = path.join("mapping.toml");
		let mapping = load_manifest(&mapping).change_context_lazy(error)?;

		Ok(Release {
			path: path.to_path_buf(),
			manifest,
			mapping,
		})
	}

	pub fn path(&self) -> &Path {
		&self.path
	}
}

impl PartialEq for Release {
	fn eq(&self, other: &Self) -> bool {
		self.manifest.id == other.manifest.id
	}
}

impl Eq for Release {}

impl Hash for Release {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.manifest.id.hash(state);
	}
}

impl Display for Release {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.manifest.id.fmt(f)
	}
}

#[derive(Debug, Deserialize)]
pub struct ReleaseManifest {
	#[serde(flatten)]
	pub id: ReleaseIdentifier,

	pub discs: Vec<Disc>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct ReleaseIdentifier {
	pub year: u16,
	pub catalog_number: String,
	pub media_type: String,
	pub audio_channels: String,
	pub provenance: String,
}

impl Display for ReleaseIdentifier {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let Self {
			year,
			catalog_number,
			media_type,
			audio_channels,
			provenance,
		} = self;
		write!(
			f,
			"({year}) {catalog_number} [{media_type}, {audio_channels}, {provenance}]"
		)
	}
}

#[derive(Debug, Error)]
pub enum ReleaseError {
	#[error("Could not load release at {path:?}")]
	Load { path: PathBuf },
}
