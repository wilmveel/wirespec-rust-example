#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}_${ARCH}" in
  Linux_x86_64)  PLATFORM="linuxX64" ;;
  Darwin_arm64)  PLATFORM="macosArm64" ;;
  Darwin_x86_64) PLATFORM="macosX64" ;;
  *) echo "Unsupported platform: ${OS}_${ARCH}" >&2; exit 1 ;;
esac

WIRESPEC="$SCRIPT_DIR/bin/${PLATFORM}/wirespec"
mkdir -r "$SCRIPT_DIR/target/gen"
"$WIRESPEC" convert OpenAPIV2 \
  -i "$SCRIPT_DIR/openapi/petstore.json" \
  -o "$SCRIPT_DIR/target/gen" \
  -l Rust \
  -p '' \
  --shared