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
use super::{LinkArgs, Target, TargetOptions};
use super::emscripten_base::{cmd};

pub fn target() -> Result<Target, String> {
    let mut post_link_args = LinkArgs::new();
    post_link_args.insert(LinkerFlavor::Em,
                          vec![String::literally("-s"),
                               String::literally("WASM=1"),
                               String::literally("-s"),
                               String::literally("ASSERTIONS=1"),
                               String::literally("-s"),
                               String::literally("ERROR_ON_UNDEFINED_SYMBOLS=1"),
                               String::literally("-g3")]);

    let opts = TargetOptions {
        linker: cmd("emcc"),

        dynamic_linking: false,
        executables: true,
        // Today emcc emits two files - a .js file to bootstrap and
        // possibly interpret the wasm, and a .wasm file
        exe_suffix: String::literally(".js"),
        linker_is_gnu: true,
        link_env: vec![("EMCC_WASM_BACKEND".to_string(), String::literally("1"))],
        allow_asm: false,
        obj_is_bitcode: true,
        is_like_emscripten: true,
        max_atomic_width: Some(32),
        post_link_args,
        target_family: Some(String::literally("unix")),
        .. Default::default()
    };
    Ok(Target {
        llvm_target: String::literally("wasm32-unknown-unknown"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        target_os: String::literally("emscripten"),
        target_env: String::literally(""),
        target_vendor: String::literally("unknown"),
        data_layout: String::literally("e-m:e-p:32:32-i64:64-n32:64-S128"),
        arch: String::literally("wasm32"),
        linker_flavor: LinkerFlavor::Em,
        options: opts,
    })
}
