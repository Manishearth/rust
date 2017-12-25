// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{LinkArgs, TargetOptions};
use std::default::Default;

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    args.insert(LinkerFlavor::Msvc,
                vec![String::literally("/NOLOGO"),
                     String::literally("/NXCOMPAT")]);

    TargetOptions {
        function_sections: true,
        linker: String::literally("link.exe"),
        dynamic_linking: true,
        executables: true,
        dll_prefix: String::literally(""),
        dll_suffix: String::literally(".dll"),
        exe_suffix: String::literally(".exe"),
        staticlib_prefix: String::literally(""),
        staticlib_suffix: String::literally(".lib"),
        target_family: Some(String::literally("windows")),
        is_like_windows: true,
        is_like_msvc: true,
        pre_link_args: args,
        crt_static_allows_dylibs: true,
        crt_static_respected: true,

        .. Default::default()
    }
}
