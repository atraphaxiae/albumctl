// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod dir;
mod filesystem;
mod manifest;
mod model;
mod result;
mod run;
mod source;

use std::process::ExitCode;

use crate::run::run;

fn main() -> ExitCode {
	if let Err(e) = run() {
		eprintln!("{e:?}");
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}
