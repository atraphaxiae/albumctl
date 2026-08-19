// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

pub struct SourceTree {
	dir: PathBuf,
	config_file: PathBuf,
	albums: Vec<AlbumTree>,
}

pub struct AlbumTree {
	dir: PathBuf,
	manifest_file: PathBuf,
	release: Vec<ReleaseTree>,
}

pub struct ReleaseTree {
	dir: PathBuf,
	manifest_file: PathBuf,
}
