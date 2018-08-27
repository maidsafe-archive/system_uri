// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

#![allow(unsafe_code, trivial_numeric_casts)]

use app::App;

use errors::Error;
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
    fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const u8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: u8,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSSetDefaultHandlerForURLScheme(scheme: CFStringRef, bundle_id: CFStringRef);
}

// helper to hand over strings to macos
fn convert_to_cfstring(content: &str) -> CFStringRef {
    unsafe {
        CFStringCreateWithBytes(
            kCFAllocatorDefault,
            content.as_ptr(),
            content.len() as CFIndex,
            0x0800_0100 as CFStringEncoding,
            false as u8,
            kCFAllocatorNull,
        )
    }
}

/// Open a given URI.
pub fn open(uri: String) -> Result<(), Error> {
    let output = Command::new("open").arg(uri).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Executing open failed. See terminal output for errors.".into())
    }
}

/// Register the given App for the given schemes.
///
/// `app` should contain all fields necessary for registering URIs on all systems. `schemes` should
/// provide a list of schemes (the initial part of a URI, like `https`).
pub fn install(app: &App, schemes: &[String]) -> Result<(), Error> {
    let bundle_id = convert_to_cfstring(app.bundle_id.as_str());
    for scheme in schemes {
        // FIXME: do we have any way to learn this failed?
        unsafe {
            LSSetDefaultHandlerForURLScheme(convert_to_cfstring(scheme.as_str()), bundle_id);
        }
    }
    Ok(())
}
