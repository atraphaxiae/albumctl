// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use error_stack::ResultExt;
use thiserror::Error;

use crate::cli::{Cli, Command};
use crate::commands::{build, check, init};
use crate::result::Result;

pub fn run() -> Result<(), RunError> {
	let args = Cli::parse();

	match args.command {
		Command::Init { path } => init(&path).change_context(RunError)?,
		Command::Check { path } => check(&path).change_context(RunError)?,
		Command::Build { path } => build(&path).change_context(RunError)?,
	}

	Ok(())
}

#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;
