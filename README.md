# HTTPS Everywhere Library

## Building build environment

    docker build -t https-everywhere-lib .

## Running build environment

    docker run -it --rm -v $(pwd):/opt https-everywhere-lib

## Building library

Run the aforementioned build environment.  In that shell:

    wasm-pack build -t no-modules

## Testing the library

We [need to](https://github.com/rustwasm/wasm-bindgen/issues/1525) add a special flag before testing in node.  This should change some time in the future.

    WBINDGEN_I_PROMISE_JS_SYNTAX_WORKS_IN_NODE=1 wasm-pack test --node
