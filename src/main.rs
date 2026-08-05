// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod album;
mod cli;
mod commands;
mod filesystem;
mod manifest;
mod project;
mod release;
mod result;
mod run;
mod tracklist;

use std::process::ExitCode;

use error_stack::{Report, fmt::ColorMode};

use crate::run::run;

fn main() -> ExitCode {
	Report::set_color_mode(ColorMode::Color);

	match run() {
		Ok(()) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("{e:?}");
			ExitCode::FAILURE
		}
	}
}
