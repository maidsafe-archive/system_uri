// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

/// The internal structure for an App. All fields are required in order to support Linux, OSX, and
/// Windows.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct App {
    /// Our apps bundle ID. On OSX, applications must be packaged as bundles in order to register a
    /// custom URI.
    pub bundle_id: String,
    /// Path to execute, including optional parameters.
    pub exec: String,
    /// Vendor name.
    pub vendor: String,
    /// The display name of the application.
    pub name: String,
    /// An optional icon, only supported on some platforms.
    pub icon: Option<String>,
}

impl App {
    /// Create a new app.
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
