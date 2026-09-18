// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use blake3::Hash;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	filesystem::{copy_file, delete, ensure_dir, move_file},
	module::{Disc, File, Metadata},
	result::Result,
};

pub fn build_unit(
	hash: &Hash,
	unit_dir: &Path,
	metadata: &Metadata,
	tracklist: &[Disc],
	files: &[File],
	output_dir: &Path,
) -> Result<Vec<PathBuf>, UnitBuildError> {
	let error = || UnitBuildError {
		dir: unit_dir.to_path_buf(),
	};

	let artist = get_unit_field(metadata, unit_dir, "artist").change_context_lazy(error)?;
	let year = get_unit_field(metadata, unit_dir, "year").change_context_lazy(error)?;
	let album = get_unit_field(metadata, unit_dir, "album").change_context_lazy(error)?;
	let release_year =
		get_unit_field(metadata, unit_dir, "release_year").change_context_lazy(error)?;
	let catalog_number =
		get_unit_field(metadata, unit_dir, "catalog_number").change_context_lazy(error)?;
	let media_type = get_unit_field(metadata, unit_dir, "media_type").change_context_lazy(error)?;
	let audio_channels =
		get_unit_field(metadata, unit_dir, "audio_channels").change_context_lazy(error)?;
	let provenance = get_unit_field(metadata, unit_dir, "provenance").change_context_lazy(error)?;

	let unit_build_dir = output_dir.join(format!(".albumctl/{hash}"));
	delete(&unit_build_dir).change_context_lazy(error)?;
	ensure_dir(&unit_build_dir).change_context_lazy(error)?;

	let unit_output_dir = output_dir.join(format!(
		"{artist} - ({year}) {album}/({release_year}) {catalog_number} [{media_type}, {audio_channels}, {provenance}]"
	));
	ensure_dir(&unit_output_dir).change_context_lazy(error)?;

	let mut unit_output_files = Vec::new();
	for File {
		file: original_file,
		disc_number,
		track_number,
	} in files
	{
		let title = get_track_field(tracklist, *disc_number, *track_number, "title")
			.change_context_lazy(error)?;
		let consistent_filename = format!("{disc_number}.{track_number:02} {title}");

		let original_file_full = unit_dir.join(&original_file);
		let copied_file_full = unit_build_dir.join(&original_file);
		let build_file_full = unit_build_dir
			.join(&consistent_filename)
			.with_added_extension(copied_file_full.extension().unwrap_or_default());
		let output_file_full = unit_output_dir
			.join(&consistent_filename)
			.with_added_extension(copied_file_full.extension().unwrap_or_default());

		copy_file(&original_file_full, &copied_file_full).change_context_lazy(error)?;
		move_file(&copied_file_full, &build_file_full).change_context_lazy(error)?;
		move_file(&build_file_full, &output_file_full).change_context_lazy(error)?;

		unit_output_files.push(output_file_full);
	}

	delete(&unit_build_dir).change_context_lazy(error)?;
	Ok(unit_output_files)
}

fn get_track_field<'a>(
	tracklist: &'a [Disc],
	disc_number: usize,
	track_number: usize,
	field: &str,
) -> Result<&'a str, MetadataError> {
	let error = || MetadataError::TrackFieldMissing {
		disc_number,
		track_number,
		field: field.to_string(),
	};

	let value = tracklist[disc_number - 1].tracks[track_number - 1]
		.metadata
		.get(field)
		.ok_or_else(error)?;
	Ok(value)
}

fn get_unit_field<'a>(
	metadata: &'a Metadata,
	unit_dir: &Path,
	field: &str,
) -> Result<&'a str, MetadataError> {
	let error = || MetadataError::UnitFieldMissing {
		dir: unit_dir.to_path_buf(),
		field: field.to_string(),
	};

	let value = metadata.get(field).ok_or_else(error)?;
	Ok(value)
}

#[derive(Debug, Error)]
pub enum MetadataError {
	#[error("Track {disc_number}.{track_number:02} is missing field '{field}'")]
	TrackFieldMissing {
		disc_number: usize,
		track_number: usize,
		field: String,
	},

	#[error("Unit {dir:?} is missing field '{field}'")]
	UnitFieldMissing { dir: PathBuf, field: String },
}

#[derive(Debug, Error)]
#[error("Could not build unit {dir:?}")]
pub struct UnitBuildError {
	dir: PathBuf,
}
