#!/bin/sh
# stage.sh TARGET [STAGE_DIR]
set -eu
cd "$(dirname "$0")/.."
target=${1:?specify a Rust target}
[ "$target" != armhf ] || target=armv7-unknown-linux-musleabihf

stage=${2:-build/stage}
target_dir=${CARGO_TARGET_DIR:-target}
bin=$target_dir/$target/release/sudoku-kindle
[ -x "$bin" ] || { echo "Build $bin before staging." >&2; exit 1; }
install -d "$stage/sudoku/bin" "$stage/documents"
install -m 755 "$bin" "$stage/sudoku/bin/sudoku-kindle"
install -m 755 deploy/Sudoku.sh "$stage/documents/Sudoku.sh"
