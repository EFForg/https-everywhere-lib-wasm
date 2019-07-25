#!/bin/bash

if [ "$TEST" == "wasm-pack-test" ]; then
  docker run -e WBINDGEN_I_PROMISE_JS_SYNTAX_WORKS_IN_NODE=1 -it --rm -v $(pwd):/opt https-everywhere-lib-wasm wasm-pack test --node
fi

if [ "$TEST" == "deterministic-build-output" ]; then
  docker run -it --rm -v $(pwd):/opt https-everywhere-lib-wasm bash -c 'wasm-pack build -t no-modules && test "`git diff pkg/https_everywhere_lib_wasm* | wc -l`" == "0"'
fi
