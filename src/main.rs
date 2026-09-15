// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod manifest;
mod module;
mod result;
mod run;

use std::process::ExitCode;

use crate::run::run;

fn main() -> ExitCode {
	match run() {
		Ok(()) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("{e:?}");
			ExitCode::FAILURE
		}
	}
}
