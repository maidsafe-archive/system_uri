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
use ffi_utils::{ErrorCode, FFI_RESULT_OK, FfiResult, catch_unwind_cb, from_c_str,
                vec_clone_from_raw_parts};

use libc::c_char;
use std::ffi::CStr;
use std::os::raw::c_void;


/// Open the given URI on this system.
#[no_mangle]
pub unsafe extern "C" fn open_uri(
    uri: *const c_char,
    user_data: *mut c_void,
    o_cb: extern "C" fn(*mut c_void, *const FfiResult),
) {
    catch_unwind_cb(user_data, o_cb, || -> Result<()> {
        let uri = from_c_str(uri)?;
        rust_open(uri)?;
        o_cb(user_data, FFI_RESULT_OK);
        Ok(())
    })
}

/// Install the given App definition for each scheme URI on the system.
/// Schemes are a comma delimited list of schemes.
#[no_mangle]
pub unsafe extern "C" fn install(
    bundle: *const c_char,
    vendor: *const c_char,
    name: *const c_char,
    exec_args: *const *const c_char,
    exec_args_len: usize,
    icon: *const c_char,
    schemes: *const c_char,
    user_data: *mut c_void,
    o_cb: extern "C" fn(*mut c_void, *const FfiResult),
) {
    catch_unwind_cb(user_data, o_cb, || -> Result<()> {
        let mut exec = String::new();
        let args = vec_clone_from_raw_parts(exec_args, exec_args_len);
        for arg in args {
            let arg_str = format!("\"{}\" ", CStr::from_ptr(arg).to_str()?);
            exec.push_str(&arg_str);
        }
        let app = App::new(
            from_c_str(bundle)?,
            from_c_str(vendor)?,
            from_c_str(name)?,
            exec.trim_right().to_owned(),
            Some(from_c_str(icon)?),
        );

        let schemes_str = from_c_str(schemes)?;

        rust_install(
            &app,
            &schemes_str
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        )?;
        o_cb(user_data, FFI_RESULT_OK);
        Ok(())
    })
}

impl ErrorCode for Error {
    fn error_code(&self) -> i32 {
        -1
    }
}
