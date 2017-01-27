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


extern crate winreg;

use self::winreg::RegKey;
use self::winreg::enums::HKEY_CURRENT_USER;
use app::App;

use errors::*;
use std::path::Path;
use std::process::Command;

// as described at https://msdn.microsoft.com/en-us/library/aa767914(v=vs.85).aspx
/// register the given App for the given schemes on Windows
pub fn install(app: App, schemes: Vec<String>) -> Result<()> {
    // but we can't write on root, we'll have to do it for the curent user only
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for protocol in schemes {
        let base_path = Path::new("Software").join("Classes").join(protocol);
        let key = hkcu.create_subkey(&base_path).chain_err(|| "could not create subkey")?;
        // set our app name as the for reference
        key.set_value("", &app.name).chain_err(|| "could not set app name key")?;
        //
        key.set_value("URL Protocol", &"").chain_err(|| "could set url protocol")?;

        let command_key = hkcu.create_subkey(&base_path.join("shell")
                .join("open")
                .join("command"))
            .chain_err(|| "could not execute open")?;
        command_key.set_value("", &format!("\"{}\" \"%1\"", app.exec))
            .chain_err(|| "could not create subkey")?
    }
    Ok(())
}

/// Open a given URI on Windows
pub fn open(uri: String) -> Result<()> {
    let _ = Command::new("explorer").arg(uri)
        .status()
        .chain_err(|| "Could not open 'explorere")?;
    // 'explorer' always comes back with a bad error code :(
    // but neither 'start' nor 'cmd /c start' seem to work...
    Ok(())
}
