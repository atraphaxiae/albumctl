// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct RootModule {
	pub output_directory: PathBuf,
	pub ffmpeg_command: Option<String>,
	pub rsgain_command: Option<String>,
	pub conversion: Option<Conversion>,
	pub replaygain: Option<Replaygain>,
	pub metadata: Metadata,
	pub children: RootChildren,
}

#[derive(Debug, Deserialize)]
pub struct Conversion {
	pub target_format: String,
	pub sample_rate: u32,
	pub sample_format: String,
}

#[derive(Debug, Deserialize)]
pub struct Replaygain {
	pub album_gain: bool,
	pub target_lufs: f32,
	pub clip_mode: ClipMode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipMode {
	Disabled,
	PositiveGain,
	AlwaysEnabled,
}

#[derive(Debug, Deserialize)]
pub struct RootChildren {
	pub modules: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Module {
	pub metadata: Metadata,
	pub discs: Option<Vec<Disc>>,
	pub children: Children,
}

pub type Metadata = BTreeMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Disc {
	pub metadata: Metadata,
	pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
	pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
	Modules { modules: Vec<PathBuf> },
	Files { files: Vec<File> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
	pub file: PathBuf,
	pub disc_number: usize,
	pub track_number: usize,
}
