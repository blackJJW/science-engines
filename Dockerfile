FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG USERNAME=dev
ARG USER_UID=1001
ARG USER_GID=1001
ARG RUST_TOOLCHAIN=stable

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl git build-essential pkg-config \
    clang lldb lld cmake ninja-build \
    python3 python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
    && mkdir -p /workspace \
    && chown -R ${USERNAME}:${USERNAME} /workspace

USER ${USERNAME}
WORKDIR /workspace

# Install Rust via rustup
ENV CARGO_HOME=/home/${USERNAME}/.cargo
ENV RUSTUP_HOME=/home/${USERNAME}/.rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_TOOLCHAIN}

# Add Cargo to PATH
ENV PATH=${CARGO_HOME}/bin:${PATH}

# Common Rust tooling (optional but useful)
RUN rustup component add rustfmt clippy

# Keep container alive for interactive use
CMD ["bash"]