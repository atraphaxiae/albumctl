// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::{
	collections::{HashMap, HashSet},
	path::{Path, PathBuf},
};

use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::{convert::Convert, replaygain::Replaygain, unit::Unit},
	manifest::load_manifest,
	module::{Children, Disc, File, Metadata, Module, RootModule},
	result::Result,
};

pub fn load_units<'a>(
	source_dir: &Path,
	root_module: &'a RootModule,
) -> Result<(Vec<Unit<'a>>, usize, usize), LoadError> {
	let error = || LoadError::RootModule {
		dir: source_dir.to_path_buf(),
	};

	let mut total_modules = 1_usize;
	let mut loaded_modules = 0_usize;

	let mut module_set = HashSet::new();
	for child_dir in &root_module.children.modules {
		if !module_set.insert(child_dir) {
			return Err(error()).attach(format!(
				"module declares child module {child_dir:?} more than once"
			));
		}
	}

	loaded_modules += 1;

	let mut loaded_units = Vec::new();
	for child_dir in &root_module.children.modules {
		let child_dir = source_dir.join(child_dir);
		if let Err(e) = recurse_modules(
			&child_dir,
			&root_module.metadata,
			None,
			root_module.convert.as_ref(),
			root_module.replaygain.as_ref(),
			&mut total_modules,
			&mut loaded_modules,
			&mut loaded_units,
		) {
			eprintln!("{e:?}\n");
		}
	}

	Ok((loaded_units, total_modules, loaded_modules))
}

fn recurse_modules<'a>(
	module_dir: &Path,
	parent_metadata: &Metadata,
	parent_tracklist: Option<&[Disc]>,
	conversion: Option<&'a Convert>,
	replaygain: Option<&'a Replaygain>,
	total_modules: &mut usize,
	loaded_modules: &mut usize,
	loaded_units: &mut Vec<Unit<'a>>,
) -> Result<(), LoadError> {
	let error = || LoadError::DescendantModule {
		dir: module_dir.to_path_buf(),
	};

	*total_modules += 1;
	let module = module_dir.join("module.toml");
	let module = load_manifest::<Module>(&module).change_context_lazy(error)?;
	let metadata = parent_metadata
		.iter()
		.chain(module.metadata.iter())
		.map(|(key, value)| (key.clone(), value.clone()))
		.collect::<Metadata>();
	let tracklist = module.discs.as_deref().or(parent_tracklist);

	match module.children {
		Children::Modules { modules } => {
			let mut module_set = HashSet::new();
			for child_dir in &modules {
				if !module_set.insert(child_dir) {
					return Err(error()).attach(format!(
						"module declares child module {child_dir:?} more than once"
					));
				}
			}

			*loaded_modules += 1;

			for child_dir in &modules {
				let child_dir = module_dir.join(child_dir);
				if let Err(e) = recurse_modules(
					&child_dir,
					&metadata,
					tracklist,
					conversion,
					replaygain,
					total_modules,
					loaded_modules,
					loaded_units,
				) {
					eprintln!("{e:?}\n");
				}
			}
		}

		Children::Files { mut files } => {
			let Some(tracklist) = tracklist else {
				return Err(error()).attach("a module lineage must contain a tracklist");
			};

			files.sort_by_key(|file| (file.disc_number, file.track_number));

			let mut files_map = HashMap::<&Path, &File>::new();
			for file in &files {
				if let Some(duplicate) = files_map.insert(&file.file, &file) {
					return Err(error()).attach(format!(
						"tracks {}.{:02} and {}.{:02} are mapped to the same source audio file {:?}",
						duplicate.disc_number,
						duplicate.track_number,
						file.disc_number,
						file.track_number,
						file.file
					));
				}
			}

			let expected = tracklist
				.iter()
				.enumerate()
				.flat_map(|(disc_number, disc)| {
					disc.tracks
						.iter()
						.enumerate()
						.map(move |(track_number, _)| (disc_number + 1, track_number + 1))
				});
			let actual = files
				.iter()
				.map(|file| (file.disc_number, file.track_number));
			if actual.ne(expected) {
				return Err(error())
					.attach("source audio file mapping doesn't match the tracklist");
			}

			*loaded_modules += 1;

			loaded_units.push(
				Unit::new(
					module_dir, &metadata, tracklist, &files, conversion, replaygain,
				)
				.change_context_lazy(error)?,
			);
		}
	}

	Ok(())
}

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Could not load root module {dir:?}")]
	RootModule { dir: PathBuf },

	#[error("Could not load descendant module {dir:?}")]
	DescendantModule { dir: PathBuf },
}
