// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::Path;

use colored::Colorize;

use crate::{build::clean::CleanError, result::Result};

pub fn summarize_build(
	source_dir: &Path,
	output_dir: &Path,
	total_modules: usize,
	loaded_modules: usize,
	total_units: usize,
	built_units: usize,
	skipped_units: usize,
	clean_result: Result<usize, CleanError>,
) {
	println!("{}", "Build completed.".green().bold());
	println!("├╴Source: {}", source_dir.display().to_string().cyan());
	println!("├╴Output: {}", output_dir.display().to_string().cyan());
	println!("│");
	println!(
		"├╴Modules: {}",
		format!("{}/{} loaded", loaded_modules, total_modules).green()
	);
	println!(
		"├╴Units:   {}, {}, {}",
		format!("{} built", built_units).green(),
		format!("{} skipped", skipped_units).cyan(),
		format!("{} failed", total_units - (built_units + skipped_units)).red(),
	);

	if let Ok(deleted_items) = clean_result {
		println!(
			"╰╴Cleanup: {}",
			format!("{} unmanaged items deleted", deleted_items).green()
		);
	} else {
		println!("╰╴Cleanup: {}", "failed".red());
	}
}
