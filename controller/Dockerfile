# Cross-compiles

FROM node:20-alpine AS client-builder

RUN npm install -g pnpm

# Copy client files
WORKDIR /app/controller/main/client
COPY main/client/package.json main/client/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# Copy client source and build
COPY main/client/ ./
RUN pnpm run build

# Rust builder stage
FROM rust:1.83-slim AS rust-builder

# Required dependencies for cross-compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libudev-dev \
    gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY controller/reflection/ ./controller/reflection/
COPY controller/reflection_derive/ ./controller/reflection_derive/
COPY controller/main/ ./controller/main/

# Copy built client files
COPY --from=client-builder /app/controller/main/client/dist ./controller/main/static

WORKDIR /app/controller/main
RUN cargo build --release --target aarch64-unknown-linux-gnu

# Runtime stage
FROM debian:bookworm-slim

# Runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libudev1 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 controller && \
    usermod -aG dialout controller

# Copy the built binary
COPY --from=rust-builder /app/controller/main/target/aarch64-unknown-linux-gnu/release/lights-controller /usr/local/bin/
COPY --from=rust-builder /app/controller/main/static /app/static

# Set capabilities for real-time scheduling
RUN setcap cap_sys_nice+ep /usr/local/bin/lights-controller

USER controller
WORKDIR /app

EXPOSE 3000

# Start the application
CMD ["lights-controller"]
