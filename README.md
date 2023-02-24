# MobileCoin build-script helper

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->![Target][target-image]<!--
-->[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->[![CodeCov Status][codecov-image]][codecov-link]<!--
-->[![GitHub Workflow Status][gha-image]][gha-link]<!--
-->[![Contributor Covenant][conduct-image]][conduct-link]

Cargo build-script assistance, from MobileCoin.

This crate provides a programmatic API for dealing with the various strings passed into build scripts via environment variables.

```rust, no_run
use mc_build_rs::Environment;

let env = Environment::new().expect("Could not parse environment");
assert_eq!(env.name(), "mc_build_rs");
```

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://mobilecoin.chat
[license-image]: https://img.shields.io/crates/l/mc-build-rs?style=flat-square
[deps-image]: https://deps.rs/repo/github/mobilecoinfoundation/build-rs/status.svg?style=flat-square
[deps-link]: https://deps.rs/repo/github/mobilecoinfoundation/build-rs
[codecov-image]: https://img.shields.io/codecov/c/github/mobilecoinfoundation/build-rs/develop?style=flat-square
[codecov-link]: https://codecov.io/gh/mobilecoinfoundation/build-rs
[gha-image]: https://img.shields.io/github/workflow/status/mobilecoinfoundation/build-rs/ci.yaml?branch=main&style=flat-square
[gha-link]: https://github.com/mobilecoinfoundation/build-rs/actions/workflows/ci.yaml?query=branch%3Amain
[target-image]: https://img.shields.io/badge/target-x86__64-blue?style=flat-square
[crate-image]: https://img.shields.io/crates/v/mc-sgx-capable.svg?style=flat-square
[crate-link]: https://crates.io/crates/mc-sgx-capable
[docs-image]: https://img.shields.io/docsrs/mc-sgx-capable?style=flat-square
[docs-link]: https://docs.rs/crate/mc-sgx-capable
[conduct-link]: CODE_OF_CONDUCT.md
[conduct-image]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square
