set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

git_revision := `git rev-parse --short HEAD`
app_version := `awk -F'"' '/^\[package\]/{p=1; next} p && /^version[[:space:]]*=/{print $2; exit}' Cargo.toml`
build_date := `date -u +%Y-%m-%dT%H:%M:%SZ`

container_runner := "docker"
container_image := "ghcr.io/sgj011206/rust_encryption"

default:
    @just --list

fmt:
    cargo fmt --all -- --check

lint:
    cargo clippy --all-targets --all-features --locked -- -D warnings

test:
    cargo llvm-cov --locked

build: fmt lint test
    cargo build --locked --release

# 一台新机器只需执行一次。
# GitHub Actions 中由 setup-buildx-action 负责，不调用此任务。
buildx-init:
    {{container_runner}} buildx inspect rust-encryption-builder >/dev/null 2>&1 || \
      {{container_runner}} buildx create \
        --name rust-encryption-builder \
        --driver docker-container \
        --use
    {{container_runner}} buildx use rust-encryption-builder
    {{container_runner}} buildx inspect --bootstrap

# 单平台本地镜像。
# 结果会载入本机 Docker，可以直接 docker run。
container-local: test
    {{container_runner}} build \
      --pull \
      --build-arg GIT_REVISION="{{git_revision}}" \
      --build-arg BUILD_DATE="{{build_date}}" \
      --build-arg VERSION="{{app_version}}" \
      -t "{{container_image}}:latest" \
      -t "{{container_image}}:{{app_version}}" \
      -f Containerfile \
      .

# 多平台发布镜像。
# 多平台结果不能直接载入传统本地镜像仓库，因此直接 push。
container:
    {{container_runner}} buildx build \
      --pull \
      --push \
      --platform linux/amd64,linux/arm64 \
      --sbom=true \
      --provenance=true \
      --build-arg GIT_REVISION="{{git_revision}}" \
      --build-arg BUILD_DATE="{{build_date}}" \
      --build-arg VERSION="{{app_version}}" \
      -t "{{container_image}}:latest" \
      -t "{{container_image}}:{{app_version}}" \
      -f Containerfile \
      .