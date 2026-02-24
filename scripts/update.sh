#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WIRESPEC_DIR="$HOME/Projects/wirespec"
CLI_DIR="$WIRESPEC_DIR/src/plugin/cli"

mkdir -p "$SCRIPT_DIR/bin/linuxX64" "$SCRIPT_DIR/bin/macosArm64" "$SCRIPT_DIR/bin/macosX64"

cp "$CLI_DIR/build/bin/linuxX64/releaseExecutable/cli.kexe" "$SCRIPT_DIR/bin/linuxX64/wirespec"
cp "$CLI_DIR/build/bin/macosArm64/releaseExecutable/cli.kexe" "$SCRIPT_DIR/bin/macosArm64/wirespec"
cp "$CLI_DIR/build/bin/macosX64/releaseExecutable/cli.kexe" "$SCRIPT_DIR/bin/macosX64/wirespec"