// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	fs::TryLockError::Error,
	path::{Path, PathBuf},
};

use blake3::Hasher;
use error_stack::ResultExt;
use postcard::to_stdvec;
use thiserror::Error;

use crate::{
	build::{prepare::BuildIndex, unit::build_unit},
	filesystem::get_mtime_size,
	manifest::save_manifest,
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
	output_dir: &Path,
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
	}

	let files = files
		.iter()
		.map(|file| file.with_new_file(&module_dir.join(&file.file)))
		.collect::<Vec<_>>();
	for file in &files {
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

	let unit_output_dir = match previous_index.get(&hash) {
		Some(dir) => dir.clone(),
		None => build_unit(&hash, module_dir, metadata, tracklist, &files, output_dir)
			.change_context_lazy(error)?,
	};
	current_index.insert(hash, unit_output_dir);
	save_manifest(index_file, current_index).change_context_lazy(error)?;

	*successful_units += 1;
	Ok(())
}

#[derive(Debug, Error)]
#[error("Could not prepare incremental build for {dir:?}")]
pub struct IncrementalBuildError {
	dir: PathBuf,
}
