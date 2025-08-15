# Cross-compiles

FROM node:20-alpine AS build-client-stage

    RUN npm install -g pnpm

    # Copy client files
    WORKDIR /app
    COPY controller/main/bindings ./controller/main/bindings
    COPY shared/bindings ./shared/bindings
    COPY controller/main/client ./controller/main/client

    WORKDIR /app/controller/main/client
    RUN pnpm install --frozen-lockfile

    RUN pnpm run build

FROM rustlang/rust:nightly-slim AS build-rust-stage

    # Required dependencies for cross-compilation
    RUN apt-get update && apt-get install -y \
        pkg-config \
        libssl-dev \
        libudev-dev \
        gcc-aarch64-linux-gnu \
        && rm -rf /var/lib/apt/lists/*

    # ARM64 libraries
    RUN dpkg --add-architecture arm64 && \
        apt-get update && \
        apt-get install -y \
        libssl-dev:arm64 \
        libudev-dev:arm64 \
        && rm -rf /var/lib/apt/lists/*

    # Create pkg-config wrapper for cross-compilation
    RUN mkdir -p /usr/local/bin && \
        echo '#!/bin/bash\n\
export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig\n\
export PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig\n\
export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu\n\
PKG_CONFIG_ALLOW_CROSS=1\n\
exec /usr/bin/pkg-config "$@"' > /usr/local/bin/pkg-config-wrapper && \
        chmod +x /usr/local/bin/pkg-config-wrapper

    RUN rustup target add aarch64-unknown-linux-gnu

    # Set up cross-compilation environment
    ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
    ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
    ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
    ENV PKG_CONFIG_aarch64_unknown_linux_gnu=pkg-config-wrapper

    WORKDIR /app
    COPY Cargo.toml Cargo.lock ./
    COPY shared/ ./shared/
    COPY controller/ ./controller/
    # We don't build roomlightctl, but cargo complains if it's not there
    COPY roomlightsctl/ ./roomlightsctl/

    WORKDIR /app/controller/main
    RUN cargo build --release --target aarch64-unknown-linux-gnu

# Runtime stage
FROM debian:bookworm-slim

    # Runtime dependencies
    RUN apt-get update && apt-get install -y \
        ca-certificates \
        libudev1 \
        libssl3 \
        libcap2-bin \
        && rm -rf /var/lib/apt/lists/*

    RUN useradd -m -u 1000 controller && \
        usermod -aG dialout controller

    # Copy the built binary
    COPY --from=build-rust-stage /app/target/aarch64-unknown-linux-gnu/release/lights-controller /app/lights-controller
    COPY --from=build-client-stage /app/controller/main/static /app/static

    # Set capabilities for real-time scheduling
    RUN setcap cap_sys_nice+ep /app/lights-controller

    USER controller
    WORKDIR /app

    EXPOSE 3000

    ENTRYPOINT ["/app/lights-controller"]
