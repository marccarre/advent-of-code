# ------------------------------------------------------------------- Base stage
FROM rust:1.48-slim AS base
# Dependencies:
# - cargo-audit
#   `- openssl-sys
#      |- libssl-dev
#      `- pkg-config
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev=1.1.1d-0+deb10u4 \
    pkg-config=0.29-6 \
    && rm -rf /var/lib/apt/lists/*
RUN rustup component add \
    clippy \
    rustfmt
RUN cargo install --version=0.13.1 cargo-audit

ENV USER=marc
ENV UID=10000
ENV GROUP=carre
ENV GID=10001
RUN groupadd \
    --gid "${GID}" \
    "${GROUP}"
RUN useradd \
    --create-home \
    --home-dir "/home/${USER}" \
    --shell /bin/sh \
    --gid "${GID}" \
    --uid "${UID}" \
    "${USER}"
RUN chown "${USER}:${GROUP}" -R /usr/local/cargo
USER "${USER}"
WORKDIR "/home/${USER}"

COPY . .

# ------------------------------------------------------------------ Build stage
FROM base AS build
RUN cargo build

# ------------------------------------------------------------------- Lint stage
FROM base AS lint
RUN cargo fmt --all -- --check
RUN cargo clippy --all-targets --all-features -- -D warnings
RUN cargo audit

# ------------------------------------------------------------------- Test stage
FROM base AS test
RUN cargo test
