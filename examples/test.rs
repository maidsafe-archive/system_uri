// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

extern crate system_uri;
extern crate rand;

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
    let schema = format!("testschema{}", rng.gen::<u32>());

    println!("Installing ourselves under {}", schema);


    install(&app, &[schema.clone()]).and_then(|()| {

        println!("Install succeeded 😄");

        println!("Trying to open {}:test", schema);

        open(format!("{}:test", schema)).and_then(|()| {
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
