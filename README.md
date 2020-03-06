# system_uri

Desktop System App URI registration handler

|Crate|Documentation|Linux/macOS|Windows|Issues|
|:---:|:-----------:|:--------:|:-----:|:----:|
|[![](http://meritbadge.herokuapp.com/system_uri)](https://crates.io/crates/system_uri)|[![Documentation](https://docs.rs/system_uri/badge.svg)](https://docs.rs/system_uri)|[![Build Status](https://travis-ci.com/maidsafe/system_uri.svg?branch=master)](https://travis-ci.com/maidsafe/system_uri)|[![Build status](https://ci.appveyor.com/api/projects/status/qpnwete63eakcipn/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/system-uri/branch/master)|[![Stories in Ready](https://badge.waffle.io/maidsafe/system_uri.png?label=ready&title=Ready)](https://waffle.io/maidsafe/system_uri)|

| [MaidSafe website](https://maidsafe.net) | [SAFE Dev Forum](https://forum.safedev.org) | [SAFE Network Forum](https://safenetforum.org) |
|:----------------------------------------:|:-------------------------------------------:|:----------------------------------------------:|

## Test Instructions

`system_uri` bridges requests for the three major desktop platforms to register URI-scheme handlers and open URIs external through one simple interface. As this only works in tight integration with the system it is running on, this crate doesn't come with unit test but integration test through examples.

To use it:
```
cargo build
cargo run --example test
```

## Configuration

If you don't need the FFI-interface, you can disable it by disabling the `ffi`-feature in your `cargo.toml` like so:

```
[dependencies.system_uri]
version = "*"
default-features = false
```

## License

This SAFE Network library is dual-licensed under the Modified BSD ([LICENSE-BSD](LICENSE-BSD) https://opensource.org/licenses/BSD-3-Clause) or the MIT license ([LICENSE-MIT](LICENSE-MIT) http://opensource.org/licenses/MIT) at your option.

## Contributing

Want to contribute? Great :tada:

There are many ways to give back to the project, whether it be writing new code, fixing bugs, or just reporting errors. All forms of contributions are encouraged!

For instructions on how to contribute, see our [Guide to contributing](CONTRIBUTING.md).
