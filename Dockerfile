FROM rust:1.38
LABEL maintainer="William Budington <bill@eff.org>"

COPY --from=node:10.16-slim /usr/local/bin/node /usr/local/bin/node
COPY --from=node:10.16-slim /usr/local/bin/npm /usr/local/bin/npm

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
