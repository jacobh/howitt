#!/usr/bin/env bash
set -euo pipefail

# Apple's clang has no Wasm backend. Select Homebrew LLVM for this target only;
# Linux builds can use their normal Wasm-capable clang/llvm-ar.
if [[ "$(uname -s)" == Darwin ]]; then
  llvm_prefix="$(brew --prefix llvm)"
  export CC_wasm32_unknown_unknown="${CC_wasm32_unknown_unknown:-$llvm_prefix/bin/clang}"
  export AR_wasm32_unknown_unknown="${AR_wasm32_unknown_unknown:-$llvm_prefix/bin/llvm-ar}"
fi

cd "$(dirname "$0")/.."
target="${1:-howitt-web}"
case "$target" in
  howitt-web) cargo metadata --format-version 1 --locked | bun scripts/prepare-timezone-assets.ts ;;
  howitt-worker) ;;
  *) echo "Unknown Worker target: $target" >&2; exit 1 ;;
esac
cd "src/bin/$target"
worker-build --profile worker --locked
