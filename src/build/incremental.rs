// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use blake3::Hasher;
use error_stack::ResultExt;
use postcard::to_stdvec;
use thiserror::Error;

use crate::{
	build::prepare::BuildIndex,
	filesystem::get_mtime_size,
	module::{Disc, File, Metadata},
	result::Result,
};

pub fn incremental_build(
	module_dir: &Path,
	metadata: &Metadata,
	tracklist: &[Disc],
	files: &[File],
	previous_index: &BuildIndex,
	current_index: &mut BuildIndex,
	index_file: &Path,
	total_units: &mut usize,
	successful_units: &mut usize,
) -> Result<(), IncrementalBuildError> {
	let error = || IncrementalBuildError {
		dir: module_dir.to_path_buf(),
	};

	*total_units += 1;

	let mut hasher = Hasher::new();
	hasher.update(
		&to_stdvec(metadata)
			.change_context_lazy(error)
			.attach_with(|| "while serializing metadata to binary")?,
	);
	hasher.update(
		&to_stdvec(tracklist)
			.change_context_lazy(error)
			.attach_with(|| "while serializing tracklist to binary")?,
	);
	for file in files {
		hasher.update(&to_stdvec(file).change_context_lazy(error).attach_with(|| {
			format!(
				"while serializing source file info of {:?} to binary",
				file.file
			)
		})?);
		hasher.update(
			&to_stdvec(&get_mtime_size(&file.file).change_context_lazy(error)?)
				.change_context_lazy(error)
				.attach_with(|| {
					format!(
						"while serializing mtime and size of {:?} to binary",
						file.file
					)
				})?,
		);
	}
	let hash = hasher.finalize();

	todo!();

	*successful_units += 1;

	Ok(())
}

#[derive(Debug, Error)]
#[error("Could not build unit {dir:?}")]
pub struct IncrementalBuildError {
	dir: PathBuf,
}
