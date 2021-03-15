// Contains modified code from https://github.com/alexcrichton/cc-rs
//
// Modification by:
// Copyright 2021 Sergei Solodovnikov
//
// Original by:
// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// `get_cpp_link_stdlib` function was taken to make it public.
use std::env;

// See license on the top
/// Returns the default C++ standard library for the current target: `libc++`
/// for OS X and `libstdc++` for anything else.
pub fn get_cpp_link_stdlib(target: &str) -> Option<String> {
    if let Ok(stdlib) = env::var("CXXSTDLIB") {
        if stdlib.is_empty() {
            None
        } else {
            Some(stdlib)
        }
    } else {
        if target.contains("msvc") {
            None
        } else if target.contains("apple") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("openbsd") {
            Some("c++".to_string())
        } else if target.contains(super::ANDROID) {
            Some("c++_shared".to_string())
        } else {
            Some("stdc++".to_string())
        }
    }
}