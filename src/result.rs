// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::result;

use error_stack::Report;

pub type Result<T, E> = result::Result<T, Report<E>>;
