# System URI - Change Log

## [0.4.0]
- Use pointers instead of passing plain structures (e.g. for `FfiResult`)
- Change the `install` function to take the `exec` command as an array of arguments rather than string
- Use rust 1.22.1 stable / 2017-12-03 nightly
- rustfmt 0.9.0 and clippy-0.0.175

## [0.3.0]
- Rename `open` to `open_uri`.

## [0.2.3]
- Fix success callback invocation in the `open` and `install` FFI functions
- Fix the `install` function on Windows: `exec` call parameters weren't quoted and didn't work properly

## [0.2.2]
- Update `ffi_utils` to 0.3.0

## [0.2.1]
- Change the format of version changing PR titles to not be a past-tense

## [0.2.0]
- Deploy to AWS
- Correct Linux schema casing to lower case
- Use native ShellExecuteW in Windows for opening uri's

## [0.1.0]
- Include change log
- Include Cargo.lock
- Use rust 1.19 stable / 2017-07-20 nightly
- rustfmt 0.9.0 and clippy-0.0.144
- Replace -Zno-trans with cargo check
- Make appveyor script using fixed version of stable

## [0.0.1]
- Initial implementation
