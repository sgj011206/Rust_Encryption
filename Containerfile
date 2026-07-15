# syntax=docker/dockerfile:1

# ------------------------------
# Stage 1. Build the application
# ------------------------------
FROM rust:1.96.0 AS builder

WORKDIR /app

# 只复制编译所必需的文件。
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 使用锁文件中的精确依赖版本。
# 同时准备最终运行时需要的二进制文件和可写数据目录。
RUN cargo build --locked --release \
    && mkdir -p /runtime/data \
    && cp /app/target/release/rust_encryption /runtime/rust_encryption \
    && touch /runtime/data/.keep

# ------------------------------
# Stage 2. Build the runtime image
# ------------------------------
#FROM dhi.io/debian-base:trixie
FROM debian:trixie-slim

ARG GIT_REVISION="unknown"
ARG BUILD_DATE="unknown"
ARG VERSION="0.0.0"

LABEL org.opencontainers.image.title="rust_encryption" \
      org.opencontainers.image.description="Simple file encryption/decryption CLI tool in Rust using AES-256-GCM" \
      org.opencontainers.image.url="https://github.com/sgj011206/Rust_Encryption" \
      org.opencontainers.image.source="https://github.com/sgj011206/Rust_Encryption" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${GIT_REVISION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.authors="sgj011206"

# DHI 运行时使用 UID/GID 65532 的 nonroot 用户。
COPY --from=builder --chown=65532:65532 \
     /runtime/rust_encryption \
     /app/rust_encryption

COPY --from=builder --chown=65532:65532 \
     /runtime/data \
     /data

WORKDIR /data

# 显式声明非特权用户，即使基础镜像默认已经是 nonroot。
USER 65532:65532

ENTRYPOINT ["/app/rust_encryption"]

# 不传参数时显示帮助，而不是让 Clap 因缺少子命令而报错。
CMD ["help"]