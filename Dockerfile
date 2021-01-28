FROM rust:1.45
MAINTAINER William Budington "bill@eff.org"

WORKDIR /root
RUN curl -O https://nodejs.org/dist/v12.18.3/node-v12.18.3-linux-x64.tar.xz
RUN tar -Jxvf node-v12.18.3-linux-x64.tar.xz
RUN ln -s /root/node-v12.18.3-linux-x64/bin/node /usr/local/bin/node
RUN ln -s /root/node-v12.18.3-linux-x64/bin/npm /usr/local/bin/npm

WORKDIR /opt
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
ADD Cargo.toml ./
ADD Cargo.lock ./
ADD lib-core ./lib-core
RUN mkdir src
RUN touch src/lib.rs
RUN wasm-pack build
RUN cargo doc
RUN cargo test
RUN echo "wasm-pack build -t no-modules" > /root/.bash_history
