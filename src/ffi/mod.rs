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

#[macro_use]
mod macros;
mod helper;

use libc::int32_t;
use super::{App, open as rust_open, install as rust_install};

#[no_mangle]
/// open the given URI on this system
pub unsafe extern "C" fn open(uri: *const u8, uri_len: usize) -> int32_t {
    helper::catch_unwind_i32(|| {
        ffi_try!(rust_open(ffi_try!(helper::c_utf8_to_str(uri, uri_len)).to_owned()
            ).and_then(|()| Ok(0)))
    })
}

#[no_mangle]
/// install the given App definition for each scheme URI on the system
/// schemes are a comma delimited list of schemes
pub unsafe extern "C" fn install(bundle_id: *const u8,
                                 bundle_id_len: usize,
                                 exec: *const u8,
                                 exec_len: usize,
                                 vendor: *const u8,
                                 vendor_len: usize,
                                 name: *const u8,
                                 name_len: usize,
                                 icon: *const u8,
                                 icon_len: usize,
                                 schemes: *const u8,
                                 schemes_len: usize)
                                 -> int32_t {
    helper::catch_unwind_i32(|| {
        let app = App::new(ffi_try!(helper::c_utf8_to_str(bundle_id, bundle_id_len)).to_owned(),
                           ffi_try!(helper::c_utf8_to_str(exec, exec_len)).to_owned(),
                           ffi_try!(helper::c_utf8_to_str(vendor, vendor_len)).to_owned(),
                           ffi_try!(helper::c_utf8_to_str(name, name_len)).to_owned(),
                           Some(ffi_try!(helper::c_utf8_to_str(icon, icon_len)).to_owned()));

        let schemes = ffi_try!(helper::c_utf8_to_str(schemes, schemes_len)).to_owned();
        ffi_try!(rust_install(app, schemes.split(',')
                     .map(|s| s.to_string())
                     .collect::<Vec<String>>())
                .and_then(|()| Ok(0)))
    })
}
