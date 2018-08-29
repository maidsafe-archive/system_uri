// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

extern crate ffi_utils;
extern crate rand;
extern crate system_uri;
#[macro_use]
extern crate unwrap;

use ffi_utils::base64_encode;
use rand::Rng;
use std::env;
use std::process::exit;
use std::{thread, time};

use system_uri::{install, open, App, SystemUriError};

fn install_and_open() -> Result<(), SystemUriError> {
    let mut rng = rand::thread_rng();
    let exec = String::from(unwrap!(unwrap!(std::env::current_exe()).to_str()));
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

        ::std::process::exit(1);
    }
}
