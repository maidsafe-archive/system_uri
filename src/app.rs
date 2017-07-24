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

#[derive(Debug)]
/// The internal structure for an App
pub struct App {
    /// our apps bundle_id
    pub bundle_id: String,
    /// path to execute, including optional parameters
    pub exec: String,
    /// What's the vendor?
    pub vendor: String,
    /// the display name of the application
    pub name: String,
    /// an optional icon, only supported on some platforms
    pub icon: Option<String>,
}

impl App {
    /// create a new app
    pub fn new(
        bundle_id: String,
        vendor: String,
        name: String,
        exec: String,
        icon: Option<String>,
    ) -> Self {
        App {
            bundle_id: bundle_id,
            name: name,
            vendor: vendor,
            exec: exec,
            icon: icon,
        }
    }
}
