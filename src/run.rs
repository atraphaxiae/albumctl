// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;

use crate::result::Result;

pub fn run() -> Result<(), RunError> {
	Ok(())
}

#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;
