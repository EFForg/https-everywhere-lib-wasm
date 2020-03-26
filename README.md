# HTTPS Everywhere WASM Library

[![Build Status](https://api.travis-ci.org/efforg/https-everywhere-lib-wasm.svg?branch=master)](https://travis-ci.org/efforg/https-everywhere-lib-wasm)
[![Latest Version](https://img.shields.io/crates/v/https-everywhere-lib-wasm.svg)](https://crates.io/crates/https-everywhere-lib-wasm)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/https-everywhere-lib-wasm)

## Preparing for the build

    git submodule update --init --recursive

## Building build environment

    docker build -t https-everywhere-lib-wasm .

## Running build environment

    docker run -it --rm -v $(pwd):/opt https-everywhere-lib-wasm

## Building library

Run the aforementioned build environment.  In that shell:

    wasm-pack build -t no-modules

## Testing the library

We [need to](https://github.com/rustwasm/wasm-bindgen/issues/1525) add a special flag before testing in node.  This should change some time in the future.

    WBINDGEN_I_PROMISE_JS_SYNTAX_WORKS_IN_NODE=1 wasm-pack test --node
