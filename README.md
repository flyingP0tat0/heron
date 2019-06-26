[![Build][build-img]][build-url]
[![Documentation][docs-img]][docs-url]
[![Demo][demo-img]][demo-url]

# heron

> A library implementing [Heron's method][wikipedia-heron] to find a square root.

## Usage

```rust
use heron::heron;

let square = 25.0;
let precision = 0.00001;

let root = heron(square, precision);

println!("square root: {}", root);
```

## Examples

This repository contains two examples.

### CLI

This program waits for the user to input a suqare and returns its suqare root with the given precision.

```
cargo run --example cli

square:
25     

absolute precision:
0.000001

square root: 5
```

### Web (WASM)

See [here][demo-url] for a live version.

The same principle applies to the second example, which works in the browser.

[`cargo-web`][cargo-web] might be used to build it:

```
cargo web start -p heron_web
```

[build-img]: https://travis-ci.com/janbaudisch/heron.svg?branch=master
[build-url]: https://travis-ci.com/janbaudisch/heron
[docs-img]: https://img.shields.io/badge/docs-master-blue.svg?colorB=4d76ae
[docs-url]: https://janbaudisch.dev/heron/heron
[demo-img]: https://img.shields.io/badge/demo-live-green.svg
[demo-url]: https://janbaudisch.dev/heron/demo
[wikipedia-heron]: https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method
[cargo-web]: https://github.com/koute/cargo-web
