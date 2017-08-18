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

extern crate system_uri;
extern crate rand;
extern crate ffi_utils;

use ffi_utils::base64_encode;
use rand::Rng;
use std::{thread, time};
use std::env;
use std::process::exit;


use system_uri::{App, SystemUriError, install, open};

fn install_and_open() -> Result<(), SystemUriError> {
    let mut rng = rand::thread_rng();
    let exec = String::from(std::env::current_exe().unwrap().to_str().unwrap());
    let app = App::new(
        "net.maidsafe.example".to_string(),
        "MaidSafe Ltd.".to_string(),
        "Example R/W".to_string(),
        exec,
        None,
    );
    let schema = base64_encode(format!("testschema-{}", rng.gen::<u32>()).as_bytes());

    println!("Installing ourselves under {}", schema);

    install(&app, &[schema.clone()]).and_then(|()| {
        println!("Install succeeded 😄");

        println!("Trying to open {}:testABC", schema);

        open(format!("{}:testABC", schema)).and_then(|()| {
            println!("Open succeeded 😄, everything is fine 🎉!");
            Ok(())
        })
    })
}

fn main() {
    if let Some(url) = env::args().nth(1) {
        println!(
            "Being started with {} as first parameter. Yay 🎉. Closing in 3",
            url
        );
        thread::sleep(time::Duration::from_secs(1));
        println!("2");
        thread::sleep(time::Duration::from_secs(1));
        println!("1");
        thread::sleep(time::Duration::from_secs(1));
        println!("Good bye!");
        exit(0);
    }

    if let Err(ref e) = install_and_open() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
