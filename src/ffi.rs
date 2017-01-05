// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net
// Commercial License, version 1.0 or later, or (2) The General Public License
// (GPL), version 3, depending on which licence you accepted on initial access
// to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project
// generally, you agree to be bound by the terms of the MaidSafe Contributor
// Agreement, version 1.0.
// This, along with the Licenses can be found in the root directory of this
// project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network
// Software distributed under the GPL Licence is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
//
// Please review the Licences for the specific language governing permissions
// and limitations relating to use of the SAFE Network Software.

#![allow(unsafe_code)]

use ffi_utils::{ErrorCode, FfiString, catch_unwind_error_code};
use super::{App, install as rust_install, open as rust_open};
use super::errors::*;

/// open the given URI on this system
#[no_mangle]
pub unsafe extern "C" fn open(uri: FfiString) -> i32 {
    catch_unwind_error_code(|| {
        let uri = uri.to_string()?;
        rust_open(uri)
    })
}

#[no_mangle]
/// install the given App definition for each scheme URI on the system
/// schemes are a comma delimited list of schemes
pub unsafe extern "C" fn install(bundle_id: FfiString,
                                 vendor: FfiString,
                                 name: FfiString,
                                 exec: FfiString,
                                 icon: FfiString,
                                 schemes: FfiString)
                                 -> i32 {
    catch_unwind_error_code(|| {
        let app = App::new(bundle_id.to_string()?,
                           vendor.to_string()?,
                           name.to_string()?,
                           exec.to_string()?,
                           Some(icon.to_string()?));

        let schemes = schemes.to_string()?;

        rust_install(app,
                     schemes.split(',')
                         .map(|s| s.to_string())
                         .collect())
    })
}

impl ErrorCode for Error {
    fn error_code(&self) -> i32 {
        -1
    }
}
