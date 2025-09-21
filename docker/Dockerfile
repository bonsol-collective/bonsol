ARG FLATC_VERSION=24.3.25
ARG RUST_VERSION=1.89.0
ARG SOLANA_VERSION=1.18.22
ARG ANCHOR_VERSION=0.31.1
ARG NODE_VERSION=22.19.0

FROM ubuntu:24.04 AS flatc-build

ARG FLATC_VERSION # Need to redeclare globals

WORKDIR /flatbuffers/flatbuffers-${FLATC_VERSION}

RUN apt-get update \
&&  apt-get install -y \
      build-essential \
      cmake \
      ca-certificates \
      curl \
      git \
      gnupg2 

ADD https://github.com/google/flatbuffers/archive/refs/tags/v${FLATC_VERSION}.tar.gz v${FLATC_VERSION}.tar.gz

RUN tar xvf v${FLATC_VERSION}.tar.gz \
  &&  cd flatbuffers-${FLATC_VERSION}/ \
  &&  cmake -G "Unix Makefiles" \
  &&  make -j \
  &&  make install \
  &&  strip /usr/local/bin/flatc

FROM ubuntu:24.04

ARG RUST_VERSION=1.89.0
ARG SOLANA_VERSION=1.18.22
ARG ANCHOR_VERSION=0.31.1
ARG NODE_VERSION=22.19.0

COPY --from=flatc-build /usr/local/bin /usr/local/include /usr/local/lib /usr/local/bin

# Deps
RUN apt-get update \
  && apt-get install -y \
    build-essential git \
    pkg-config \
    libudev-dev llvm libclang-dev \
    protobuf-compiler libssl-dev curl

# Install Rust
ENV PATH="/root/.local/bin:/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup install 1.89.0 && rustup default 1.89.0
RUN rustup install nightly 
RUN rustup component add rustfmt rust-analyzer clippy
RUN cargo install cargo-binstall

# Risc0
RUN cargo binstall cargo-risczero

# pre-commit
RUN curl -LsSf https://astral.sh/uv/install.sh | sh \
  && uv tool install pre-commit

# NodeJS
ENV NVM_DIR=/root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash \
    && . "$NVM_DIR/nvm.sh" \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# Solana
ENV SOLANA_VERSION=v${SOLANA_VERSION}  
RUN sh -c "$(curl -sSfL https://release.anza.xyz/${SOLANA_VERSION}/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Anchor
RUN cargo install --git https://github.com/coral-xyz/anchor avm \
    && avm install $ANCHOR_VERSION \
    && avm use $ANCHOR_VERSION

