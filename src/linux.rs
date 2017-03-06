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

use app::App;

use errors::*;
use std::ascii::AsciiExt;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use xdg_basedir::dirs::get_data_home;

/// Open a given URI on Linux systems
pub fn open<S: Into<String>>(uri: S) -> Result<()> {
    let uri = uri.into();

    let output = Command::new("xdg-open")
        .arg(uri.clone())
        .output()
        .chain_err(|| "Could not execute xdg-open")?;

    if output.status.success() {
        Ok(())
    } else {
        Err(
            format!(
                "Executing `xdg-open {}` failed: {}",
                uri,
                String::from_utf8_lossy(&output.stdout)
            ).into(),
        )
    }

}

fn clean_string(input: &str) -> String {
    input.replace(".", "").replace("/", "").to_ascii_lowercase()
}

/// register the given App for the given schemes on Linux
pub fn install(app: &App, schemes: &[String]) -> Result<()> {
    let home = get_data_home().chain_err(|| "Home directory not found")?;
    let ascii_name = format!(
        "{}-{}.desktop",
        clean_string(&app.vendor).as_str(),
        clean_string(&app.name).as_str()
    );

    let mut desktop_target = PathBuf::new();
    desktop_target.push(home);
    desktop_target.push("applications");

    let apps_dir = desktop_target.clone();

    create_dir_all(apps_dir.clone()).chain_err(
        || "Could not create app directory",
    )?;

    desktop_target.push(ascii_name.clone());
    let mut f = File::create(desktop_target.as_path()).chain_err(
        || "Could not create app desktop file",
    )?;
    let schemes_list = schemes
        .iter()
        .map(|s| match s.matches(char::is_uppercase).next() {
            Some(_) => {
                println!("[WARN] system-uri: converting schema '{}' to lowercase", s);
                format!("x-scheme-handler/{}", s.to_lowercase())
            }
            None => format!("x-scheme-handler/{}", s),
        })
        .collect::<Vec<String>>();

    f.write_fmt(format_args!(
        include_str!("./template.desktop"),
        name = app.name,
        exec = app.exec,
        // app.icon.unwrap_or("".to_string()),
        mime_types = schemes_list.join(";")
    )).chain_err(|| " Could not write app desktop file")?;

    let status = Command::new("update-desktop-database")
        .arg(apps_dir)
        .status()
        .chain_err(|| "Could not run update-desktop-database")?;


    for scheme in schemes_list {
        let _ = Command::new("xdg-mime")
            .arg("default")
            .arg(ascii_name.clone())
            .arg(scheme)
            .status()
            .chain_err(|| "Could not run xdg-mime")?;
    }

    if status.success() {
        Ok(())
    } else {
        Err(
            ("Executing update-desktop-database failed. See terminal output for errors.").into(),
        )
    }
}
