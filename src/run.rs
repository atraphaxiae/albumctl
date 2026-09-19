// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	build::build,
	cli::{Cli, Command},
	result::Result,
};

pub fn run() -> Result<(), RunError> {
	println!();

	let args = Cli::parse();

	match args.command {
		Command::Build { dir } => build(&dir).change_context(RunError)?,
	}

	Ok(())
}

#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;
