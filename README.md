# system_uri

Desktop System App URI registration handler

**Maintainer:** Spandan Sharma (spandan.sharma@maidsafe.net)

|Crate|Documentation|Linux/OS X|Windows|Issues|
|:---:|:-----------:|:--------:|:-----:|:----:|
|[![](http://meritbadge.herokuapp.com/system_uri)](https://crates.io/crates/system_uri) | [![Documentation](https://docs.rs/system_uri/badge.svg)](https://docs.rs/system_uri) | [![Build Status](https://travis-ci.org/maidsafe/system_uri.svg?branch=master)](https://travis-ci.org/maidsafe/system_uri) | [![Build status](https://ci.appveyor.com/api/projects/status/qpnwete63eakcipn/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/system-uri/branch/master) | [![Stories in Ready](https://badge.waffle.io/maidsafe/system_uri.png?label=ready&title=Ready)](https://waffle.io/maidsafe/system_uri)|


| [SAFE Dev Forum](https://forum.safedev.org) | [SAFE Network Forum](https://safenetforum.org) |
|:------:|:-------:|:-------:|:-------:|


## Test Instructions

`system_uri` bridges requests for the three major desktop platforms to register URI-scheme handlers and open URIs external through one simple interface. As this only works in tight integration with the system it is running on, this crate doesn't come with unittest but integration test through examples.

To use it with the Mock:
```
cargo build
cargo run --example test
```

## Configuration

If you don't need the FFI-interface, you can disable it by disabeling the `ffi`-feature in your `cargo.toml` like so:

```
[dependencies.system_uri]
version = "*"
default-features = false
```

## License

Licensed under either of

* the MaidSafe.net Commercial License, version 1.0 or later ([LICENSE](LICENSE))
* the General Public License (GPL), version 3 ([COPYING](COPYING) or http://www.gnu.org/licenses/gpl-3.0.en.html)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the MaidSafe Contributor Agreement, version 1.1 ([CONTRIBUTOR]
(CONTRIBUTOR)), shall be dual licensed as above, and you agree to be bound by the terms of the
MaidSafe Contributor Agreement, version 1.1.