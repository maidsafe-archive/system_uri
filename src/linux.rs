// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::app::App;

use crate::errors::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use xdg_basedir::dirs::get_data_home;

/// Open a given URI.
pub fn open<S: Into<String>>(uri: S) -> Result<(), Error> {
    let uri = uri.into();

    let output = Command::new("xdg-open").arg(uri.clone()).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::XdgOpenError(
            uri,
            String::from_utf8_lossy(&output.stdout).to_string(),
        ))
    }
}

/// Clean URI for xdg-open.
fn clean_string(input: &str) -> String {
    input.replace(".", "").replace("/", "").to_ascii_lowercase()
}

/// Register the given App for the given schemes.
///
/// `app` should contain all fields necessary for registering URIs on all systems. `schemes` should
/// provide a list of schemes (the initial part of a URI, like `https`).
pub fn install(app: &App, schemes: &[String]) -> Result<(), Error> {
    let home = get_data_home().map_err(|_| Error::Unexpected("Home directory not found"))?;
    let ascii_name = format!(
        "{}-{}.desktop",
        clean_string(&app.vendor).as_str(),
        clean_string(&app.name).as_str()
    );

    let mut desktop_target = PathBuf::new();
    desktop_target.push(home);
    desktop_target.push("applications");

    let apps_dir = desktop_target.clone();

    create_dir_all(apps_dir.clone())?;

    desktop_target.push(ascii_name.clone());
    let mut f = File::create(desktop_target.as_path())?;
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
    ))
    .map_err(|_| Error::Unexpected("Could not write app desktop file"))?;

    let status = Command::new("update-desktop-database")
        .arg(apps_dir)
        .status()
        .map_err(|_| Error::Unexpected("Could not run update-desktop-database"))?;

    for scheme in schemes_list {
        let _ = Command::new("xdg-mime")
            .arg("default")
            .arg(ascii_name.clone())
            .arg(scheme)
            .status()?;
    }

    if status.success() {
        Ok(())
    } else {
        Err(("Executing update-desktop-database failed. See terminal output for errors.").into())
    }
}
