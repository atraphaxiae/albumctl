// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use thiserror::Error;

use crate::cli::Cli;
use crate::result::Result;

pub fn run() -> Result<(), RunError> {
	let args = Cli::parse();
	match args.command {
		_ => todo!(),
	}

	Ok(())
}

#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;
