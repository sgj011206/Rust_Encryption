#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONTENT_DIR="${ROOT_DIR}/docs/content"
STATIC_DIR="${ROOT_DIR}/docs/static"

README_FILE="${ROOT_DIR}/README.md"
SPEC_FILE="${ROOT_DIR}/spec.md"
ICON_FILE="${ROOT_DIR}/icon.svg"

for required_file in "${README_FILE}" "${SPEC_FILE}"; do
  if [[ ! -f "${required_file}" ]]; then
    echo "Required file not found: ${required_file}" >&2
    exit 1
  fi
done

mkdir -p "${CONTENT_DIR}" "${STATIC_DIR}"

# 使用 README.md 生成网站首页。
# 删除 README 中已有的一级标题，避免与 Hugo 页面标题重复。
{
  cat <<'FRONT_MATTER'
+++
title = 'Rust Encryption'
menus = 'main'
weight = 10
description = 'Rustで書かれたシンプルなファイル暗号化・復号ツール'
+++

FRONT_MATTER

  awk '
    BEGIN {
      removed_title = 0
    }

    !removed_title && /^#[[:space:]]+Rust_Encryption[[:space:]]*$/ {
      removed_title = 1
      next
    }

    {
      print
    }
  ' "${README_FILE}"
} > "${CONTENT_DIR}/_index.md"

# 使用 spec.md 生成独立的 Specification 页面。
{
  cat <<'FRONT_MATTER'
+++
title = 'Specification'
menus = 'main'
weight = 20
description = 'Rust Encryption の仕様'
+++

FRONT_MATTER

  cat "${SPEC_FILE}"
} > "${CONTENT_DIR}/spec.md"

# 将根目录图标复制到 Hugo static。
if [[ -f "${ICON_FILE}" ]]; then
  cp "${ICON_FILE}" "${STATIC_DIR}/icon.svg"
fi

echo "Hugo content synchronized successfully."