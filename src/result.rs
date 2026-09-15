// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use error_stack::Report;

pub type Result<T, E> = std::result::Result<T, Report<E>>;
