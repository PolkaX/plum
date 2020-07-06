// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_types::MethodNum;

///
#[allow(non_upper_case_globals)]
pub const MethodSend: MethodNum = 0;
///
#[allow(non_upper_case_globals)]
pub const MethodConstructor: MethodNum = 1;
/// TODO fin: remove this once canonical method numbers are finalized
#[allow(non_upper_case_globals)]
pub const MethodPlaceholder: MethodNum = 1 << 30;
