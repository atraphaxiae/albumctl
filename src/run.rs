// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use error_stack::ResultExt;
use thiserror::Error;

use crate::{
	cli::{Cli, Command},
	command::{build, check},
	result::Result,
};

pub fn run() -> Result<(), RunError> {
	let error = || RunError;

	let args = Cli::parse();
	match args.command {
		Command::Build { dir } => check(&dir).change_context_lazy(error)?,
		Command::Check { dir } => build(&dir).change_context_lazy(error)?,
	}

	Ok(())
}

#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;
