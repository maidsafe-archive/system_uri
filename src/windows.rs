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

extern crate winreg;
extern crate winapi;

use self::winapi::windef::HWND;
use self::winapi::winnt::LPCWSTR;
use self::winreg::RegKey;
use self::winreg::enums::HKEY_CURRENT_USER;
use app::App;

use errors::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

#[link(name = "shell32")]
extern "system" {
    pub fn ShellExecuteW(
        hwnd: HWND,
        lpOperation: LPCWSTR,
        lpFile: LPCWSTR,
        lpParameters: LPCWSTR,
        lpDirectory: LPCWSTR,
        nShowCmd: i32,
    ) -> i32;
}


// as described at https://msdn.microsoft.com/en-us/library/aa767914(v=vs.85).aspx
/// register the given App for the given schemes on Windows
pub fn install(app: &App, schemes: &[String]) -> Result<()> {
    // but we can't write on root, we'll have to do it for the curent user only
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for protocol in schemes {
        let base_path = Path::new("Software").join("Classes").join(protocol);
        let key = hkcu.create_subkey(&base_path).chain_err(
            || "could not create subkey",
        )?;
        // set our app name as the for reference
        key.set_value("", &app.name).chain_err(
            || "could not set app name key",
        )?;
        //
        key.set_value("URL Protocol", &"").chain_err(
            || "could set url protocol",
        )?;

        let command_key = hkcu.create_subkey(&base_path.join("shell").join("open").join("command"))
            .chain_err(|| "could not execute open")?;
        command_key
            .set_value("", &format!("\"{}\" \"%1\"", app.exec))
            .chain_err(|| "could not create subkey")?
    }
    Ok(())
}

/// Open a given URI on Windows
#[allow(unsafe_code)]
pub fn open(uri: String) -> Result<()> {
    let err = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            to_wide_chars("open").as_ptr(),
            to_wide_chars(&(uri.replace("\n", "%0A"))).as_ptr(),
            ptr::null(),
            ptr::null(),
            winapi::SW_SHOWNORMAL,
        )
    };
    if err < 32 {
        Err(
            format!("Executing open failed with error_code {}.", err).into(),
        )
    } else {
        Ok(())
    }
}
