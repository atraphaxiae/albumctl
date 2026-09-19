// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	path::{Path, PathBuf},
	process::Command,
};

use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::result::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Convert {
	pub ffmpeg_command: String,
	pub target_format: String,
	pub sample_rate: u32,
	pub sample_format: String,
}

impl Convert {
	pub fn process(&self, input_file: &Path, output_file: &Path) -> Result<(), ConvertError> {
		// ffmpeg -i <input_file> -ar <sample_rate> -sample_fmt <sample_format> <output_file>
		let result = Command::new(&self.ffmpeg_command)
			.arg("-i")
			.arg(input_file)
			.arg("-ar")
			.arg(self.sample_rate.to_string())
			.arg("-sample_fmt")
			.arg(&self.sample_format)
			.arg(output_file)
			.output()
			.change_context_lazy(|| ConvertError::Execute {
				file: input_file.to_path_buf(),
			})?;

		if !result.status.success() {
			return Err(ConvertError::Ffmpeg {
				file: input_file.to_path_buf(),
				stderr: String::from_utf8_lossy(&result.stderr).to_string(),
			}
			.into());
		}

		Ok(())
	}
}

#[derive(Debug, Error)]
pub enum ConvertError {
	#[error("Could not execute convert for file {file:?}")]
	Execute { file: PathBuf },

	#[error("ffmpeg convert failed for file {file:?}:\n{stderr}")]
	Ffmpeg { file: PathBuf, stderr: String },
}
