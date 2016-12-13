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


use std::env;
use std::ascii::AsciiExt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;

use errors::*;
use app::App;

/// Open a given URI on Linux systems
pub fn open(uri: String) -> Result<()> {
    let status = Command::new("xdg-open").arg(uri)
        .status()
        .chain_err(|| "Could not execute xdg-open")?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Executing xdg-open failed. See terminal output for errors.").into())
    }

}

/// register the given App for the given schemes on Linux
pub fn install(app: App, schemes: Vec<String>) -> Result<()> {
    let home = env::home_dir().ok_or("Home directory not found")?;
    let ascii_name = format!("{}-{}",
                             app.vendor.as_str().to_ascii_lowercase(),
                             app.name.as_str().to_ascii_lowercase());

    let mut desktop_target = PathBuf::new();
    desktop_target.push(home);
    desktop_target.push(".local");
    desktop_target.push("share");
    desktop_target.push("applications");

    let apps_dir = desktop_target.clone();

    create_dir_all(apps_dir.clone()).chain_err(|| "Could not create app directory")?;

    desktop_target.push(ascii_name + ".desktop");
    let mut f =
        File::create(desktop_target.as_path()).chain_err(|| "Could not create app desktop file")?;
    let schemes_list = schemes.iter()
        .map(|s| format!("x-scheme-handler/{};", s))
        .collect::<Vec<String>>()
        .join("");

    f.write_fmt(format_args!(include_str!("./template.desktop"),
                                name = app.name,
                                exec = app.exec,
                                // app.icon.unwrap_or("".to_string()),
                                mime_types = schemes_list))
        .chain_err(|| " Could not write app desktop file")?;

    let status = Command::new("update-desktop-database").arg(apps_dir)
        .status()
        .chain_err(|| "Could not run update-desktop-database")?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Executing update-desktop-database failed. See terminal output for errors.")
            .into())
    }
}
