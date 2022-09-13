# Try Box

[![GitHub Release](https://img.shields.io/github/release/try-box/try-box?color=brightgreen)](https://github.com/try-box/try-box/releases)

English | [简体中文](./README-CN.md)

*Try Box* This crate provides an odd set of tools for Rust programming:

## Crates
The main `try-box` crate just [re-exports](src/lib.rs) tools from
smaller subcrates:

* [`stream-ext`](stream-ext)
  An extension trait for Streams that provides a variety of convenient combinator functions.
* [`queue-ext`](queue-ext)
  The extended feature of the queue can convert the pop() function of the queue to Stream and the push() function to Sender.


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
try-box = "0.1"
```


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.