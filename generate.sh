#!/usr/bin/env bash
# Regenerates src/generated from openapi/petstore.json with a released Wirespec CLI.
set -euo pipefail

WIRESPEC_VERSION="${WIRESPEC_VERSION:-0.22.6}"

cd "$(dirname "$0")"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)  PLATFORM=linuxX64 ;;
  Darwin-arm64)  PLATFORM=macosArm64 ;;
  Darwin-x86_64) PLATFORM=macosX64 ;;
  *) echo "No Wirespec CLI release for $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac

WIRESPEC="bin/wirespec-$WIRESPEC_VERSION"
if [ ! -x "$WIRESPEC" ]; then
  mkdir -p bin
  curl -fsSL -o "$WIRESPEC" \
    "https://github.com/flock-community/wirespec/releases/download/v$WIRESPEC_VERSION/$PLATFORM.kexe"
  chmod +x "$WIRESPEC"
fi

rm -rf src/generated
"$WIRESPEC" convert OpenAPIV2 \
  --input openapi/petstore.json \
  --output src/generated \
  --language Rust \
  --package '' \
  --shared

# The generators are not part of the module tree in src/generated/mod.rs, so they are never compiled.
rm -rf src/generated/generator
