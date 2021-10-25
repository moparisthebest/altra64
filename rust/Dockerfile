
# to build and test:
# docker build -t altra64-rust-dev . && docker run --rm -v "$HOME/.cargo/registry:/root/.cargo/registry" -v "$(pwd):/build" -it altra64-rust-dev

# to use to compile altra64 (or other n64 stuff I guess)
# docker run --rm -v "$HOME/.cargo/registry:/root/.cargo/registry" -v "$(pwd):/build" -it altra64-rust-dev ./build.sh

FROM moparisthebest/altra64-dev

ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/local/libdragon/bin:/root/.cargo/bin

RUN apt-get -y install curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --profile minimal --default-toolchain nightly --component rust-src && \
    apt-get -y purge curl && \
    cargo install cbindgen && \
    rm -rf ~/.cargo/registry/
