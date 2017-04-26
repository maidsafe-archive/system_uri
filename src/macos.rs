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

#![allow(unsafe_code, trivial_numeric_casts)]

use app::App;

use errors::*;
use libc;
use std::process::Command;

#[repr(C)]
struct __CFString(libc::c_void);
type CFStringRef = *const __CFString;

type CFAllocatorRef = *const libc::c_void;
type CFIndex = libc::c_long;
type CFStringEncoding = u32;


#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFAllocatorDefault: CFAllocatorRef;
    static kCFAllocatorNull: CFAllocatorRef;
    fn CFStringCreateWithBytes(alloc: CFAllocatorRef,
                               bytes: *const u8,
                               numBytes: CFIndex,
                               encoding: CFStringEncoding,
                               isExternalRepresentation: u8,
                               contentsDeallocator: CFAllocatorRef)
                               -> CFStringRef;
}


#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSSetDefaultHandlerForURLScheme(scheme: CFStringRef, bundle_id: CFStringRef);
}


// helper to hand over strings to macos
fn convert_to_cfstring(content: &str) -> CFStringRef {
    unsafe {
        CFStringCreateWithBytes(kCFAllocatorDefault,
                                content.as_ptr(),
                                content.len() as CFIndex,
                                0x08000100 as CFStringEncoding,
                                false as u8,
                                kCFAllocatorNull)
    }
}


/// Open a given URI on MacOSX systems
pub fn open(uri: String) -> Result<()> {
    let status = Command::new("open")
        .arg(uri)
        .status()
        .chain_err(|| "Could not execute open")?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Executing open failed. See terminal output for errors.").into())
    }
}

/// register the given App for the given schemes on MacOSX
pub fn install(app: &App, schemes: &[String]) -> Result<()> {
    let bundle_id = convert_to_cfstring(app.bundle_id.as_str());
    for scheme in schemes {
        // FIXME: do we have any way to learn this failed?
        unsafe {
            LSSetDefaultHandlerForURLScheme(convert_to_cfstring(scheme.as_str()), bundle_id);
        }
    }
    Ok(())
}
