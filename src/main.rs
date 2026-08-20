// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod command;
mod dir;
mod filesystem;
mod manifest;
mod model;
mod result;
mod run;
mod source;

use std::process::ExitCode;

use error_stack::{Report, fmt::ColorMode};

use crate::run::run;

fn main() -> ExitCode {
	Report::set_color_mode(ColorMode::Color);

	if let Err(e) = run() {
		eprintln!("{e:?}");
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}
