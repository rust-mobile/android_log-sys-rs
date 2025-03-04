// Copyright 2016 The android_log_sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
pub mod bindings;

pub use bindings::android_LogPriority as LogPriority;
pub use bindings::*;

// Ensure linking happens against liblog.
#[link(name = "log")]
extern "C" {}
