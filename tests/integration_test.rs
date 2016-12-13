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

use rand::Rng;

use system_uri::{App, install, open};

#[test]
fn install_and_open() {
    let mut rng = rand::thread_rng();
    let exec = String::from(std::env::current_exe().unwrap().to_str().unwrap());
    println!("{:}", exec);
    let app = App::new("net.maidsafe.example".to_string(),
                       "MaidSafe".to_string(),
                       "Example".to_string(),
                       exec,
                       None);
    let schema = format!("testschema{}", rng.gen::<u32>());

    assert!(install(app, vec![schema.clone()]).is_ok());
    assert!(open(format!("{}:test", schema)).is_ok());
}
