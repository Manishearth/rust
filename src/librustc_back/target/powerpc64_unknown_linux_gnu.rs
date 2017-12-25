// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{Target, TargetResult, RelroLevel};

pub fn target() -> TargetResult {
    let mut base = super::linux_base::opts();
    base.cpu = String::literally("ppc64");
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push(String::literally("-m64"));
    base.max_atomic_width = Some(64);

    // ld.so in at least RHEL6 on ppc64 has a bug related to BIND_NOW, so only enable partial RELRO
    // for now. https://github.com/rust-lang/rust/pull/43170#issuecomment-315411474
    base.relro_level = RelroLevel::Partial;

    // see #36994
    base.exe_allocation_crate = None;

    Ok(Target {
        llvm_target: String::literally("powerpc64-unknown-linux-gnu"),
        target_endian: String::literally("big"),
        target_pointer_width: String::literally("64"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("E-m:e-i64:64-n32:64"),
        arch: String::literally("powerpc64"),
        target_os: String::literally("linux"),
        target_env: String::literally("gnu"),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
