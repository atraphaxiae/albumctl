// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod module;

use clap::Parser;

use crate::cli::Cli;

fn main() {
	let args = Cli::parse();
}
