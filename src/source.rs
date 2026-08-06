// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::album::Album;
use crate::filesystem::{
	copy_file, delete_dir, ensure_dir, list_dirs, require_absent, require_dir,
};
use crate::manifest::{load_manifest, save_manifest};
use crate::release::ResolvedTrack;
use crate::result::Result;

#[derive(Debug)]
pub struct Source {
	root: PathBuf,
	manifest: SourceManifest,
}

impl Source {
	pub fn init(path: &Path) -> Result<Self, SourceError> {
		let error = || SourceError::Init {
			path: path.to_path_buf(),
		};

		ensure_dir(path).change_context_lazy(error)?;

		let manifest_path = path.join("albumctl.toml");
		require_absent(&manifest_path).change_context_lazy(error)?;

		let manifest = SourceManifest {
			output_directory: "".into(),
		};

		save_manifest(&manifest_path, &manifest).change_context_lazy(error)?;

		Ok(Source {
			root: path.to_path_buf(),
			manifest,
		})
	}

	pub fn load(path: &Path) -> Result<Self, SourceError> {
		let error = || SourceError::Load {
			path: path.to_path_buf(),
		};

		require_dir(path).change_context_lazy(error)?;

		let manifest = path.join("albumctl.toml");
		let manifest = load_manifest(&manifest).change_context_lazy(error)?;

		Ok(Source {
			root: path.to_path_buf(),
			manifest,
		})
	}

	pub fn check(&self) -> Result<(), SourceError> {
		let error = || SourceError::Check {
			path: self.root.clone(),
		};

		let albums = self.load_albums().change_context(error())?;
		for album in albums {
			album.load_releases().change_context(error())?;
		}

		Ok(())
	}

	pub fn build(&self) -> Result<PathBuf, SourceError> {
		let error = || SourceError::Build {
			path: self.root.clone(),
		};

		let outdir = &self.manifest.output_directory;
		delete_dir(outdir).change_context_lazy(error)?;
		ensure_dir(outdir).change_context_lazy(error)?;

		let albums = self.load_albums().change_context_lazy(error)?;
		for album in albums {
			println!("Building album \"{album}\"");

			let album_path = outdir.join(album.to_string());
			ensure_dir(&album_path).change_context_lazy(error)?;

			let releases = album.load_releases().change_context(error())?;
			for release in releases {
				println!("╰╴Building release \"{release}\"");

				let release_path = album_path.join(release.to_string());
				ensure_dir(&release_path).change_context_lazy(error)?;

				let tracks = release.resolve_tracks();
				for track in tracks {
					let ResolvedTrack {
						disc_number,
						track_number,
						track,
						file,
					} = track;

					let mut output_file = release_path.join(format!(
						"{}.{:02} {}",
						disc_number + 1,
						track_number + 1,
						track.title
					));

					if let Some(extension) = file.extension() {
						output_file.add_extension(extension);
					}

					copy_file(&file, &output_file).change_context_lazy(error)?;
				}
			}
		}

		Ok(self.manifest.output_directory.clone())
	}

	pub fn load_albums(&self) -> Result<HashSet<Album>, SourceError> {
		let error = || SourceError::LoadAlbums {
			path: self.root.clone(),
		};

		let mut albums = HashSet::<Album>::new();
		for dir in list_dirs(&self.root).change_context_lazy(error)? {
			let album = Album::load(&dir).change_context_lazy(error)?;
			if let Some(original) = albums.get(&album) {
				Err(error()).attach(formatdoc!(
					r#"
						duplicate albums of "{album}" found at:
							- {}
							- {}
					"#,
					album.path().display(),
					original.path().display()
				))?;
			}
			albums.insert(album);
		}

		Ok(albums)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceManifest {
	pub output_directory: PathBuf,
}

#[derive(Debug, Error)]
pub enum SourceError {
	#[error("Could not initialize source at {path:?}")]
	Init { path: PathBuf },

	#[error("Could not load source at {path:?}")]
	Load { path: PathBuf },

	#[error("Detected errors in source at {path:?}")]
	Check { path: PathBuf },

	#[error("Could not build from source at {path:?}")]
	Build { path: PathBuf },

	#[error("Could not load albums from source at {path:?}")]
	LoadAlbums { path: PathBuf },
}
