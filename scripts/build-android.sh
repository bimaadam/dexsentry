#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
JNI_LIBS_DIR="${ROOT_DIR}/android/core-bridge/src/main/jniLibs"
KOTLIN_OUTPUT_DIR="${ROOT_DIR}/android/core-bridge/build/generated/uniffi"
HOST_LIBRARY="${ROOT_DIR}/target/release/libdexsentry_ffi.so"

mkdir -p "${JNI_LIBS_DIR}" "${KOTLIN_OUTPUT_DIR}"

cargo build --release -p dexsentry-ffi

cargo run -p dexsentry-ffi --features bindgen --bin uniffi-bindgen -- \
    generate --library "${HOST_LIBRARY}" \
    --config "${ROOT_DIR}/uniffi.toml" \
    --language kotlin \
    --out-dir "${KOTLIN_OUTPUT_DIR}"

cargo ndk \
    -t arm64-v8a \
    -t armeabi-v7a \
    -t x86_64 \
    -o "${JNI_LIBS_DIR}" \
    build --release -p dexsentry-ffi
