// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	fs::{read_to_string, write},
	path::{Path, PathBuf},
};

use error_stack::ResultExt;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use toml::{from_str, to_string_pretty};

use crate::result::Result;

pub fn load_manifest<T: DeserializeOwned>(file: &Path) -> Result<T, ManifestError> {
	let error = || ManifestError::LoadManifest {
		file: file.to_path_buf(),
	};

	let data = read_to_string(file)
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {file:?}"))?;

	let manifest = from_str(&data)
		.change_context_lazy(error)
		.attach_with(|| "while parsing TOML data")?;

	Ok(manifest)
}

pub fn save_manifest<T: Serialize>(file: &Path, manifest: &T) -> Result<(), ManifestError> {
	let error = || ManifestError::SaveManifest {
		file: file.to_path_buf(),
	};

	let data = to_string_pretty(manifest)
		.change_context_lazy(error)
		.attach_with(|| "while serializing to TOML")?;

	write(file, data)
		.change_context_lazy(error)
		.attach_with(|| format!("while writing to {file:?}"))?;

	Ok(())
}

#[derive(Debug, Error)]
pub enum ManifestError {
	#[error("Could not load manifest from {file:?}")]
	LoadManifest { file: PathBuf },

	#[error("Could not save manifest to {file:?}")]
	SaveManifest { file: PathBuf },
}
