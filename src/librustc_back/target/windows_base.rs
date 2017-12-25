// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
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
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Gcc, vec![
            // And here, we see obscure linker flags #45. On windows, it has been
            // found to be necessary to have this flag to compile liblibc.
            //
            // First a bit of background. On Windows, the file format is not ELF,
            // but COFF (at least according to LLVM). COFF doesn't officially allow
            // for section names over 8 characters, apparently. Our metadata
            // section, ".note.rustc", you'll note is over 8 characters.
            //
            // On more recent versions of gcc on mingw, apparently the section name
            // is *not* truncated, but rather stored elsewhere in a separate lookup
            // table. On older versions of gcc, they apparently always truncated th
            // section names (at least in some cases). Truncating the section name
            // actually creates "invalid" objects [1] [2], but only for some
            // introspection tools, not in terms of whether it can be loaded.
            //
            // Long story short, passing this flag forces the linker to *not*
            // truncate section names (so we can find the metadata section after
            // it's compiled). The real kicker is that rust compiled just fine on
            // windows for quite a long time *without* this flag, so I have no idea
            // why it suddenly started failing for liblibc. Regardless, we
            // definitely don't want section name truncation, so we're keeping this
            // flag for windows.
            //
            // [1] - https://sourceware.org/bugzilla/show_bug.cgi?id=13130
            // [2] - https://code.google.com/p/go/issues/detail?id=2139
            String::literally("-Wl,--enable-long-section-names"),

            // Tell GCC to avoid linker plugins, because we are not bundling
            // them with Windows installer, and Rust does its own LTO anyways.
            String::literally("-fno-use-linker-plugin"),

            // Always enable DEP (NX bit) when it is available
            String::literally("-Wl,--nxcompat"),

            // Do not use the standard system startup files or libraries when linking
            String::literally("-nostdlib"),
        ]);

    let mut late_link_args = LinkArgs::new();
    late_link_args.insert(LinkerFlavor::Gcc, vec![
        String::literally("-lmingwex"),
        String::literally("-lmingw32"),
        String::literally("-lgcc"), // alas, mingw* libraries above depend on libgcc
        String::literally("-lmsvcrt"),
        String::literally("-luser32"),
        String::literally("-lkernel32"),
    ]);

    TargetOptions {
        // FIXME(#13846) this should be enabled for windows
        function_sections: false,
        linker: String::literally("gcc"),
        dynamic_linking: true,
        executables: true,
        dll_prefix: String::literally(""),
        dll_suffix: String::literally(".dll"),
        exe_suffix: String::literally(".exe"),
        staticlib_prefix: String::literally(""),
        staticlib_suffix: String::literally(".lib"),
        no_default_libraries: true,
        target_family: Some(String::literally("windows")),
        is_like_windows: true,
        allows_weak_linkage: false,
        pre_link_args,
        pre_link_objects_exe: vec![
            String::literally("crt2.o"),    // mingw C runtime initialization for executables
            String::literally("rsbegin.o"), // Rust compiler runtime initialization, see rsbegin.rs
        ],
        pre_link_objects_dll: vec![
            String::literally("dllcrt2.o"), // mingw C runtime initialization for dlls
            String::literally("rsbegin.o"),
        ],
        late_link_args,
        post_link_objects: vec![
            String::literally("rsend.o")
        ],
        custom_unwind_resume: true,

        .. Default::default()
    }
}
