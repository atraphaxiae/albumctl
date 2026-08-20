// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <atraphaxiae@gmail.com>
// SPDX-License-Identifier: MPL-2.0

macro_rules! success {
	($($arg:tt)*) => {{
		use colored::Colorize;
		println!("{}", format!($($arg)*).green());
	}};
}

pub(crate) use success;
