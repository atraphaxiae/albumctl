// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;

use crate::cli::Cli;

mod cli;

fn main() {
	let args = Cli::parse();
}
