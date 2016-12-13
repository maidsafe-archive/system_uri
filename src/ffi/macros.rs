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

// from https://github.com/maidsafe/safe_core/blob/master/src/ffi/macros.rs

macro_rules! ffi_try {
    ($result:expr) => {
        match $result {
            Ok(value)  => value,
            Err(error) => {
                let decorator = ::std::iter::repeat('-').take(50).collect::<String>();
                let err_str = format!("{:?}", error);
                let err_code = 1;
                // FIXME: this should become info! again, when this has been merged back
                println!(
                    "\nFFI cross-boundary error propagation:\n {}\n| **ERRNO: {}** {}\n {}\n\n",
                    decorator, err_code, err_str, decorator);
                return err_code
            },
        }
    }
}
