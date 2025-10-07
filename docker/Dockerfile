ARG FLATC_VERSION=24.3.25
ARG RUST_VERSION=1.89.0
ARG SOLANA_VERSION=2.3.11
ARG ANCHOR_VERSION=0.31.1
ARG NODE_VERSION=22.19.0
ARG CUDA_VERSION=13

FROM ubuntu:24.04 AS flatc-build

ARG FLATC_VERSION # Need to redeclare globals

WORKDIR /flatbuffers/flatbuffers-${FLATC_VERSION}

RUN apt-get update \
  && apt-get install --no-install-recommends -y \
      build-essential \
      cmake \
      ca-certificates \
      curl \
      git \
      gnupg2 \
      unzip \
      rsync \
  && rm -rf /var/lib/apt/lists/*

ADD https://github.com/google/flatbuffers/archive/refs/tags/v${FLATC_VERSION}.tar.gz v${FLATC_VERSION}.tar.gz

RUN tar xvf v${FLATC_VERSION}.tar.gz \
  &&  cd flatbuffers-${FLATC_VERSION}/ \
  &&  cmake -G "Unix Makefiles" \
  &&  make -j \
  &&  make install \
  &&  strip /usr/local/bin/flatc

FROM ubuntu:24.04

ARG RUST_VERSION
ARG SOLANA_VERSION
ARG ANCHOR_VERSION
ARG NODE_VERSION
ARG CUDA_VERSION

COPY --from=flatc-build /usr/local/bin /usr/local/bin
COPY --from=flatc-build /usr/local/include /usr/local/include
COPY --from=flatc-build /usr/local/lib /usr/local/lib

# Deps
RUN apt-get update \
  && apt-get install --no-install-recommends -y \
    build-essential git libsodium23 \
    pkg-config ca-certificates \
    libudev-dev llvm libclang-dev \
    protobuf-compiler libssl-dev curl \
  && rm -rf /var/lib/apt/lists/*

# Install Rust
ENV PATH="/root/.local/share/solana/install/active_release/bin:/root/.local/bin:/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
  && rustup install ${RUST_VERSION} && rustup default ${RUST_VERSION} \
  && rustup install nightly \
  && rustup component add rustfmt rust-analyzer clippy \
  && cargo install cargo-binstall

# Get CUDA-ed
RUN curl -O https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2404/x86_64/cuda-keyring_1.1-1_all.deb \
  && dpkg -i cuda-keyring_1.1-1_all.deb \
  && apt-get update \
  && apt-get install --no-install-recommends -y cuda-toolkit-${CUDA_VERSION} \
  && rm -rf /var/lib/apt/lists/* \
  && rm cuda-keyring_1.1-1_all.deb

ENV PATH=/usr/local/cuda/bin:${PATH}

# Risc0
RUN cargo binstall -y cargo-risczero

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

# Anchor
RUN cargo install --git https://github.com/coral-xyz/anchor avm \
    && avm install $ANCHOR_VERSION \
    && avm use $ANCHOR_VERSION
