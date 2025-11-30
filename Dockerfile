# Cross-compiles

FROM --platform=${BUILDPLATFORM} node:20-alpine AS build-client-stage

    RUN npm install -g pnpm

    # Copy client files
    WORKDIR /app
    COPY controller/main/bindings ./controller/main/bindings
    COPY shared/bindings ./shared/bindings
    COPY controller/main/client ./controller/main/client

    WORKDIR /app/controller/main/client
    RUN pnpm install --frozen-lockfile

    RUN pnpm run build

FROM --platform=${BUILDPLATFORM} rustlang/rust:nightly-slim AS build-rust-stage

    ARG TARGET_ARCH=x86_64-unknown-linux-gnu
    ARG CROSS_COMPILE=false

    # Required dependencies for cross-compilation
    RUN apt-get update && apt-get install -y \
            pkg-config \
            libssl-dev \
            build-essential \
            ca-certificates \
            libudev-dev \
            $(if [ "$CROSS_COMPILE" = "true" ]; then echo "gcc-aarch64-linux-gnu"; fi) \
        && rm -rf /var/lib/apt/lists/*

    # ARM64 libraries (only if cross-compiling)
    RUN if [ "$CROSS_COMPILE" = "true" ]; then \
        dpkg --add-architecture arm64 && \
        apt-get update && \
        apt-get install -y \
            pkg-config:arm64 \
            libssl-dev:arm64 \
            libudev-dev:arm64 \
            ca-certificates:arm64 \
        && rm -rf /var/lib/apt/lists/*; \
    fi

    # Create pkg-config wrapper for cross-compilation
    RUN if [ "$CROSS_COMPILE" = "true" ]; then \
        mkdir -p /usr/local/bin && \
        echo '#!/bin/bash\n\
export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig\n\
export PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig\n\
export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu\n\
PKG_CONFIG_ALLOW_CROSS=1\n\
exec /usr/bin/pkg-config "$@"' > /usr/local/bin/pkg-config-wrapper && \
        chmod +x /usr/local/bin/pkg-config-wrapper; \
        fi

    RUN rustup target add $TARGET_ARCH

    ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=${CROSS_COMPILE:+aarch64-linux-gnu-gcc}
    ENV CC_aarch64_unknown_linux_gnu=${CROSS_COMPILE:+aarch64-linux-gnu-gcc}
    ENV CXX_aarch64_unknown_linux_gnu=${CROSS_COMPILE:+aarch64-linux-gnu-g++}
    ENV PKG_CONFIG_aarch64_unknown_linux_gnu=${CROSS_COMPILE:+pkg-config-wrapper}

    WORKDIR /app
    COPY Cargo.toml Cargo.lock ./
    COPY shared/ ./shared/
    COPY controller/ ./controller/
    # We don't build roomlightctl, but cargo complains if it's not there
    COPY roomlightsctl/ ./roomlightsctl/

    WORKDIR /app/controller/main
    RUN cargo build --release --target $TARGET_ARCH

    RUN mv /app/target/$TARGET_ARCH/release/lights-controller /app/lights-controller

# Runtime stage
FROM arm64v8/debian:bookworm-slim

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
    COPY --from=build-rust-stage /app/lights-controller /app/lights-controller
    COPY --from=build-client-stage /app/controller/main/static /app/static

    # Set capabilities for real-time scheduling
    RUN setcap cap_sys_nice+ep /app/lights-controller

    USER controller
    WORKDIR /app

    EXPOSE 3000

    ENTRYPOINT ["/app/lights-controller"]
