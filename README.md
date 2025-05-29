# rancid-cache

A repository to better understand the varnish-cache approach to configuration. 

[![Rust](https://github.com/chotchki/rancid-cache/actions/workflows/rust.yml/badge.svg)](https://github.com/chotchki/rancid-cache/actions/workflows/rust.yml)

# MVP Approach

* Parse a given VCL program (starting with something super simple)
* * Produce an AST
* Walk the AST and construct a tokio/hyper proxy

## Research Notes

Varnish acts as a proxy server that compiles its configuration into native code using the DSL language called VCL (https://varnish-cache.org/docs/trunk/users-guide/vcl.html).

## Name

This is mimicing the varnish proxy server using Rust. Varnish is commonly an oil of some kind and if oxidized would be rancid.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.