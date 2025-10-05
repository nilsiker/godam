FROM barichello/godot-ci:4.5

# setup rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

RUN . $CARGO_HOME/env

# install godam
RUN cargo install --git https://github.com/nilsiker/godam --branch feature/25-git-support

WORKDIR /workspace

# default cmd
CMD ["godam", "--help"]
