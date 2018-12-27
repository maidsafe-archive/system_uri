// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! # System-Uri Library
//! [Project github page](https://github.com/maidsafe/system_uri)

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
    html_favicon_url = "http://maidsafe.net/img/favicon.ico",
    test(attr(forbid(warnings)))
)]
// For explanation of lint checks, run `rustc -W help` or see
// https://github.
// com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md

// inspired by
// https://github.
// com/feross/webtorrent-desktop/blob/4bb2056bc9c1a421815b97d03ffed512575dfde0/src/main/handlers.js
#![forbid(
    exceeding_bitshifts,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types,
    warnings
)]
#![deny(
    bad_style,
    deprecated,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    overflowing_literals,
    plugin_as_library,
    stable_features,
    unconditional_recursion,
    unknown_lints,
    unsafe_code,
    unused,
    unused_allocation,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_parens,
    while_true
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![allow(
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    variant_size_differences
)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(
    feature = "clippy",
    deny(clippy, unicode_not_nfc, wrong_pub_self_convention, option_unwrap_used)
)]
#![cfg_attr(
    feature = "clippy",
    allow(use_debug, too_many_arguments, needless_return)
)]

#[macro_use]
extern crate quick_error;

#[cfg(any(target_os = "macos", feature = "ffi"))]
extern crate libc;

#[cfg(target_os = "linux")]
extern crate xdg_basedir;

#[cfg(feature = "ffi")]
extern crate ffi_utils;

mod app;
pub use app::App;

mod errors {
    use ffi_utils::StringError;
    use std::io;
    use std::str::Utf8Error;

    quick_error! {
        /// System URI error variants.
        #[derive(Debug)]
        pub enum Error {
            /// IO error.
            IoError(error: io::Error) {
                description("Io error")
                display("I/O error: {}", error)
                from()
            }
            /// String error.
            StringError(error: StringError) {
                description("String error")
                display("String error: {:?}", error)
                from()
            }
            /// Utf-8 error.
            Utf8Error(error: Utf8Error) {
                description(error.description())
                display("Utf-8 error: {}", error)
                from()
            }
            #[cfg(target_os = "linux")]
            /// XDG error.
            XdgOpenError(uri: String, stdout: String) {
                description(uri)
                display("Executing `xdg-open {}` failed: {}", uri, stdout)
            }
            #[cfg(target_os = "windows")]
            /// Open error.
            ShellOpenError(code: i32) {
                display("Using ShellExecuteW to open URL failed with code {}", code)
            }
            /// Unexpected error.
            Unexpected(s: &'static str) {
                description(s)
                display("Unexpected error: {}", s)
                from()
            }
        }
    }
}

pub use errors::Error as SystemUriError;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{install, open};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{install, open};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{install, open};

/// Foreign Function call Interface to use this library
#[cfg(feature = "ffi")]
pub mod ffi;
