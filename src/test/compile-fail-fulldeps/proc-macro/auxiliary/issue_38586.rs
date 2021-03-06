// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// force-host
// no-prefer-dynamic

#![feature(proc_macro, proc_macro_lib)]
#![crate_type = "proc-macro"]

extern crate proc_macro;

#[proc_macro_derive(A)]
pub fn derive_a(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "fn f() { println!(\"{}\", foo); }".parse().unwrap()
}
