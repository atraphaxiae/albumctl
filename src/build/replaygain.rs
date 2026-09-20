// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{path::Path, process::Command};

use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::result::Result;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Replaygain {
	pub rsgain_command: String,
	pub album_gain: bool,
	pub target_lufs: f32,
	pub clip_mode: ClipMode,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipMode {
	Disabled,
	PositiveGain,
	AlwaysEnabled,
}

impl Replaygain {
	pub fn process<P: AsRef<Path>>(&self, input_files: &[P]) -> Result<(), ReplaygainError> {
		let mut command = Command::new(&self.rsgain_command);
		command.arg("custom");
		command.arg("-s");
		command.arg("i");

		if self.album_gain {
			command.arg("-a");
		}

		command.arg("-l");
		command.arg(self.target_lufs.to_string());

		command.arg("-c");
		match self.clip_mode {
			ClipMode::Disabled => command.arg("n"),
			ClipMode::PositiveGain => command.arg("p"),
			ClipMode::AlwaysEnabled => command.arg("a"),
		};

		for input_file in input_files {
			command.arg(input_file.as_ref());
		}

		let result = command
			.output()
			.change_context_lazy(|| ReplaygainError::Execute)?;
		if !result.status.success() {
			return Err(ReplaygainError::Rsgain {
				stderr: String::from_utf8_lossy(&result.stderr).to_string(),
			}
			.into());
		}

		Ok(())
	}
}

#[derive(Debug, Error)]
pub enum ReplaygainError {
	#[error("Could not execute replaygain tagging")]
	Execute,

	#[error("rsgain replaygain tagging failed:\n{stderr}")]
	Rsgain { stderr: String },
}
