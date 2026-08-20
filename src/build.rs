// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use crate::model::Model;

pub mod finalize;
pub mod normalize;
pub mod prepare;
pub mod process;

pub struct Builder {
	model: Model,
}

impl Builder {
	pub fn new(model: Model) -> Self {
		Self { model }
	}
}
