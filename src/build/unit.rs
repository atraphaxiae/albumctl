// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use blake3::{Hash, Hasher};
use error_stack::ResultExt;
use postcard::to_stdvec;
use thiserror::Error;

use crate::{
	build::{BuildIndex, convert::Convert, replaygain::Replaygain},
	filesystem::{copy_file, delete_item, ensure_dir, get_mtime_size, move_file},
	manifest::save_manifest,
	module::{Disc, File, Metadata},
	result::Result,
};

pub fn build_units(
	units: &[Unit],
	current_index: &mut BuildIndex,
	index_file: &Path,
	output_dir: &Path,
) -> usize {
	let mut built_units = 0;
	for unit in units {
		if let Err(e) = unit.build(current_index, index_file, output_dir) {
			eprintln!("{e:?}\n");
		} else {
			built_units += 1;
		}
	}

	built_units
}

#[derive(Debug)]
pub struct Unit<'a> {
	pub unit_dir: PathBuf,
	pub metadata: Metadata,
	pub tracklist: Vec<Disc>,
	pub files: Vec<File>,
	pub convert: Option<&'a Convert>,
	pub replaygain: Option<&'a Replaygain>,
	pub hash: Hash,
}

impl<'a> Unit<'a> {
	pub fn new(
		unit_dir: &Path,
		metadata: &Metadata,
		tracklist: &[Disc],
		files: &[File],
		conversion: Option<&'a Convert>,
		replaygain: Option<&'a Replaygain>,
	) -> Result<Self, UnitError> {
		let error = || UnitError::New {
			dir: unit_dir.to_path_buf(),
		};

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
				&to_stdvec(&get_mtime_size(&unit_dir.join(&file.file)).change_context_lazy(error)?)
					.change_context_lazy(error)
					.attach_with(|| {
						format!(
							"while serializing mtime and size of {:?} to binary",
							file.file
						)
					})?,
			);
		}

		hasher.update(
			&to_stdvec(&conversion)
				.change_context_lazy(error)
				.attach_with(|| "while serializing conversion configuration to binary")?,
		);
		hasher.update(
			&to_stdvec(&replaygain)
				.change_context_lazy(error)
				.attach_with(|| "while serializing replaygain configuration to binary")?,
		);

		let hash = hasher.finalize();

		Ok(Unit {
			unit_dir: unit_dir.to_path_buf(),
			metadata: metadata.clone(),
			tracklist: tracklist.to_vec(),
			files: files.to_vec(),
			convert: conversion,
			replaygain,
			hash,
		})
	}

	fn build(
		&self,
		current_index: &mut BuildIndex,
		index_file: &Path,
		output_dir: &Path,
	) -> Result<(), UnitError> {
		let error = || UnitError::Build {
			dir: self.unit_dir.to_path_buf(),
		};

		let artist = self.get_field("artist").change_context_lazy(error)?;
		let album = self.get_field("album").change_context_lazy(error)?;
		let year = self.get_field("year").change_context_lazy(error)?;

		let catalog_number = self
			.get_field("catalog_number")
			.change_context_lazy(error)?;
		let release_year = self.get_field("release_year").change_context_lazy(error)?;
		let media_type = self.get_field("media_type").change_context_lazy(error)?;
		let audio_channels = self
			.get_field("audio_channels")
			.change_context_lazy(error)?;
		let provenance = self.get_field("provenance").change_context_lazy(error)?;

		let unit_build_dir = output_dir.join(format!(".albumctl/{}", self.hash));
		ensure_dir(&unit_build_dir).change_context_lazy(error)?;

		let unit_output_dir = output_dir.join(format!(
		"{artist} - ({year}) {album}/({release_year}) {catalog_number} [{media_type}, {audio_channels}, {provenance}]"
		));
		ensure_dir(&unit_output_dir).change_context_lazy(error)?;

		let mut unit_build_files = Vec::new();
		let mut unit_output_files = Vec::new();

		// To lessen confusion, here's the var suffix scheme to use here:
		// filename: a relative file path without its extension
		// file: a relative file path with its extension (or none if it doesn't have one)
		// flle_full: a full file path with its extension

		// We are doing it this way because I'll become even more confused if I did it by
		// doing Path::file_name() or Path::parent() or whatever, and those things need
		// error handling too which is more annoying than just doing it this way
		for File {
			file: original_file,
			disc_number,
			track_number,
		} in &self.files
		{
			let title = self
				.get_track_field(*disc_number, *track_number, "title")
				.change_context_lazy(error)?;
			let consistent_filename =
				PathBuf::from(format!("{disc_number}.{track_number:02} {title}"));

			let original_file_full = self.unit_dir.join(&original_file);
			let copied_file_full = unit_build_dir.join(&original_file);

			// This is the file that we should eventually move to the unit output dir
			let mut build_file = consistent_filename.with_added_extension(
				original_file
					.extension()
					.ok_or_else(error)
					.attach_with(|| {
						format!(
							"source audio file of track {disc_number}.{track_number:02} does not have an extension"
						)
					})?,
			);
			let mut build_file_full = unit_build_dir.join(&build_file);
			let mut output_file_full = unit_output_dir.join(&build_file);

			copy_file(&original_file_full, &copied_file_full).change_context_lazy(error)?;
			move_file(&copied_file_full, &build_file_full).change_context_lazy(error)?;

			// Conversion first then tagging
			if let Some(convert) = self.convert {
				// Basically we want, for example, music.dsf -> music.dsf.flac -> music.flac
				let converted_file_full =
					build_file_full.with_added_extension(&convert.target_format);

				let renamed_file = consistent_filename.with_added_extension(&convert.target_format);
				let renamed_file_full = unit_build_dir.join(&renamed_file);

				convert
					.process(&build_file_full, &converted_file_full)
					.change_context_lazy(error)?;

				delete_item(&build_file_full).change_context_lazy(error)?;
				move_file(&converted_file_full, &renamed_file_full).change_context_lazy(error)?;

				// Because the extension changed, we need to change build_file and output_file
				build_file = renamed_file.clone();
				build_file_full = renamed_file_full;
				output_file_full = unit_output_dir.join(renamed_file);
			}

			unit_build_files.push(build_file_full);
			unit_output_files.push(output_file_full);
		}

		// Replaygain needs to be done outside the per-file loop so that album gains are properly
		// calculated
		if let Some(replaygain) = self.replaygain {
			replaygain
				.process(&unit_build_files)
				.change_context_lazy(error)?;
		}

		for (unit_build_file, unit_output_file) in
			unit_build_files.iter().zip(unit_output_files.iter())
		{
			move_file(unit_build_file, unit_output_file).change_context_lazy(error)?;
		}

		// Finally add to the current index and save it
		current_index.insert(self.hash.to_string(), unit_output_files);
		save_manifest(index_file, current_index).change_context_lazy(error)?;

		Ok(())
	}

	fn get_field(&self, field: &str) -> Result<&str, UnitError> {
		let error = || UnitError::MissingField {
			dir: self.unit_dir.to_path_buf(),
			field: field.to_string(),
		};

		let value = self.metadata.get(field).ok_or_else(error)?;
		Ok(value)
	}

	fn get_track_field(
		&self,
		disc_number: usize,
		track_number: usize,
		field: &str,
	) -> Result<&str, UnitError> {
		let error = || UnitError::MissingTrackField {
			disc_number,
			track_number,
			dir: self.unit_dir.to_path_buf(),
			field: field.to_string(),
		};

		let value = self.tracklist[disc_number - 1].tracks[track_number - 1]
			.metadata
			.get(field)
			.ok_or_else(error)?;
		Ok(value)
	}
}

#[derive(Debug, Error)]
pub enum UnitError {
	#[error("Could not load unit {dir:?}")]
	New { dir: PathBuf },

	#[error("Could not build unit {dir:?}")]
	Build { dir: PathBuf },

	#[error("Unit {dir:?} is missing field {field}")]
	MissingField { dir: PathBuf, field: String },

	#[error("Track {disc_number}.{track_number:02} of unit {dir:?} is missing field {field}")]
	MissingTrackField {
		disc_number: usize,
		track_number: usize,
		dir: PathBuf,
		field: String,
	},
}
