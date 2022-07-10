# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld cmake \
   && apt-get autoremove -y && apt-get clean -y

USER vscode

#RUN yarn global add tailwindcss
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install cargo-make
RUN rustup toolchain install nightly \
   && rustup component add rustfmt --toolchain nightly
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
