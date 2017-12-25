// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::linux_base::opts();
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push(String::literally("-m32"));
    base.max_atomic_width = Some(32);

    // see #36994
    base.exe_allocation_crate = None;

    Ok(Target {
        llvm_target: String::literally("powerpc-unknown-linux-gnu"),
        target_endian: String::literally("big"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("E-m:e-p:32:32-i64:64-n32"),
        arch: String::literally("powerpc"),
        target_os: String::literally("linux"),
        target_env: String::literally("gnu"),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
