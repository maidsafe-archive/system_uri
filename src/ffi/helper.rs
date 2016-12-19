// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

// based on https://github.com/maidsafe/safe_core/blob/master/src/ffi/helper.rs

use libc::int32_t;
use std;
use std::error::Error;
use std::panic;
use std::slice;
use super::super::SystemUriError;

pub unsafe fn c_utf8_to_str(ptr: *const u8, len: usize) -> Result<&'static str, SystemUriError> {
    std::str::from_utf8(slice::from_raw_parts(ptr, len))
        .map_err(|error| SystemUriError::from(error.description()))
}

pub fn catch_unwind_i32<F: FnOnce() -> int32_t>(f: F) -> int32_t {
    panic::catch_unwind(panic::AssertUnwindSafe(f)).unwrap_or(1)
}
