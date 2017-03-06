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

#[allow(unused_imports)]

extern crate system_uri;
extern crate rand;

use rand::Rng;
use std::env;
#[cfg(target_os = "linux")]
use std::process::Command;

use system_uri::{App, install, open};

#[cfg(target_os = "linux")]
fn check(scheme: String) {

    if env::var("TRAVIS").is_err() {
        println!("opening {}:test", scheme);
        assert!(open(format!("{}://test", scheme)).is_ok());
    } else {
        // in travis we can only check the configuration, but not open
        // as we don't actually have a desktop
        println!("in travis");

        let output = Command::new("xdg-mime")
            .arg("query")
            .arg("default")
            .arg(format!("x-scheme-handler/{}", scheme))
            .output()
            .expect("xdg-mime failed");

        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(output.stdout.as_slice()),
                   "maidsafe-example.desktop\n".to_owned());

    }
}


#[cfg(target_os = "macos")]
fn check(_: String) {
    // unfortunately registration won't work in mac unless we have a bundle
    assert!(true);
}


#[cfg(target_os = "windows")]
fn check(scheme: String) {
    assert!(open(format!("{}:test", scheme)).is_ok());
}


#[test]
fn install_and_check() {
    if let Some(url) = env::args().skip(1).next() {
        println!("Being started with {} as first parameter. Yay 🎉.", url);
        // check that the first parameter is our schema
        assert!(url.starts_with("testschema"));
    } else {
        // directly called, let's do the testing
        let mut rng = rand::thread_rng();
        let exec = String::from(std::env::current_exe().unwrap().to_str().unwrap());
        let schema = format!("testschema-ABC-{}", rng.gen::<u32>());
        println!("{:} for {}", exec, schema);
        let app = App::new("net.maidsafe.example".to_string(),
                           "MaidSafe".to_string(),
                           "Example".to_string(),
                           exec,
                           None);

        assert!(install(app, vec![schema.clone()]).is_ok());
        check(schema);
    }
}
