// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use toml::{from_str, to_string_pretty};

use crate::result::Result;

pub fn load_manifest<T: DeserializeOwned>(path: &Path) -> Result<T, ManifestError> {
	let error = || ManifestError::LoadManifest {
		path: path.to_path_buf(),
	};

	let data = read_to_string(path)
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {path:?}"))?;

	let manifest = from_str::<T>(&data)
		.change_context_lazy(error)
		.attach("while parsing TOML data")?;

	Ok(manifest)
}

pub fn save_manifest<T: Serialize>(path: &Path, manifest: &T) -> Result<(), ManifestError> {
	let error = || ManifestError::SaveManifest {
		path: path.to_path_buf(),
	};

	let data = to_string_pretty(manifest)
		.change_context_lazy(error)
		.attach("while serializing to TOML")?;

	write(path, data)
		.change_context_lazy(error)
		.attach_with(|| format!("while writing to {path:?}"))
}

#[derive(Debug, Error)]
pub enum ManifestError {
	#[error("Could not load manifest from {path:?}")]
	LoadManifest { path: PathBuf },

	#[error("Could not save manifest to {path:?}")]
	SaveManifest { path: PathBuf },
}
