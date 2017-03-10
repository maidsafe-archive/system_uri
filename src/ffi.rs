// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

#![allow(unsafe_code)]

use super::{App, install as rust_install, open as rust_open};
use super::errors::*;
use ffi_utils::{ErrorCode, catch_unwind_error_code};

use libc::c_char;
use std::ffi::CStr;


/// open the given URI on this system
#[no_mangle]
pub unsafe extern "C" fn open(uri: *const c_char) -> i32 {
    catch_unwind_error_code(|| {
                                let uri = (CStr::from_ptr(uri).to_str()?).to_owned();
                                rust_open(uri)
                            })
}

#[no_mangle]
/// install the given App definition for each scheme URI on the system
/// schemes are a comma delimited list of schemes
pub unsafe extern "C" fn install(bundle: *const c_char,
                                 vendor: *const c_char,
                                 name: *const c_char,
                                 exec: *const c_char,
                                 icon: *const c_char,
                                 schemes: *const c_char)
                                 -> i32 {
    catch_unwind_error_code(|| {
        let app = App::new((CStr::from_ptr(bundle).to_str()?).to_owned(),
                           (CStr::from_ptr(vendor).to_str()?).to_owned(),
                           (CStr::from_ptr(name).to_str()?).to_owned(),
                           (CStr::from_ptr(exec).to_str()?).to_owned(),
                           Some((CStr::from_ptr(icon).to_str()?).to_owned()));

        let schemes = (CStr::from_ptr(schemes).to_str()?).to_owned();

        rust_install(app, schemes.split(',').map(|s| s.to_string()).collect())
    })
}

impl ErrorCode for Error {
    fn error_code(&self) -> i32 {
        -1
    }
}
