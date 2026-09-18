// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod build;
mod cli;
mod filesystem;
mod manifest;
mod module;
mod result;
mod run;

use std::process::ExitCode;

use error_stack::{Report, fmt::ColorMode};

use crate::run::run;

fn main() -> ExitCode {
	Report::set_color_mode(ColorMode::Color);

	match run() {
		Ok(()) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("{e:?}\n");
			ExitCode::FAILURE
		}
	}
}
