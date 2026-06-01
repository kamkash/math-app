#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
SOURCE_DIR="${LLAMA_LIB_SOURCE_DIR:-$HOME/mathappws/llama.cpp/build-cpu/bin}"
DEST_DIR="${TAURI_LIB_DEST_DIR:-$SCRIPT_DIR/assets/libs}"
MATHAPP_LIB_SOURCE="${MATHAPP_LIB_SOURCE:-$SCRIPT_DIR/../src-cpp/target/libmathapp.dylib}"

require_tool() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "error: required tool '$1' was not found" >&2
        exit 1
    fi
}

resolve_dependency_filename() {
    local dependency="$1"
    local filename="${dependency##*/}"

    if [[ -f "$DEST_DIR/$filename" ]]; then
        printf '%s\n' "$filename"
        return 0
    fi

    # llama.cpp may record versioned install names such as libggml.0.dylib
    # while the copied files are named libggml.dylib.
    local unversioned
    unversioned=$(printf '%s\n' "$filename" | sed -E 's/(\.[0-9]+)+\.dylib$/.dylib/')
    if [[ -f "$DEST_DIR/$unversioned" ]]; then
        printf '%s\n' "$unversioned"
        return 0
    fi

    return 1
}

copy_existing_dylibs() {
    mkdir -p "$DEST_DIR"

    echo "Copying llama.cpp dylibs from $SOURCE_DIR"

    shopt -s nullglob
    local source_file filename
    for source_file in "$SOURCE_DIR"/*.dylib; do
        filename=$(basename "$source_file")

        if [[ -f "$DEST_DIR/$filename" ]]; then
            echo "  updating $filename"
            cp "$source_file" "$DEST_DIR/$filename"
        else
            echo "  skipping $filename (not present in destination)"
        fi
    done
    shopt -u nullglob
}

copy_mathapp_dylib() {
    if [[ -f "$MATHAPP_LIB_SOURCE" ]]; then
        echo "Updating libmathapp.dylib from $MATHAPP_LIB_SOURCE"
        cp "$MATHAPP_LIB_SOURCE" "$DEST_DIR/libmathapp.dylib"
    else
        echo "warning: libmathapp.dylib source not found: $MATHAPP_LIB_SOURCE" >&2
        echo "warning: keeping existing $DEST_DIR/libmathapp.dylib" >&2
    fi
}

patch_dylib() {
    local dylib="$1"
    local filename
    filename=$(basename "$dylib")

    echo "Patching $filename"
    install_name_tool -id "@loader_path/$filename" "$dylib"

    local dependency replacement
    while IFS= read -r dependency; do
        [[ "$dependency" == @rpath/*.dylib ]] || continue

        if replacement=$(resolve_dependency_filename "$dependency"); then
            echo "  $dependency -> @loader_path/$replacement"
            install_name_tool -change "$dependency" "@loader_path/$replacement" "$dylib"
        else
            echo "  warning: no bundled match for $dependency" >&2
        fi
    done < <(otool -L "$dylib" | awk 'NR > 1 { print $1 }')

    local rpath
    while IFS= read -r rpath; do
        echo "  removing rpath $rpath"
        install_name_tool -delete_rpath "$rpath" "$dylib" 2>/dev/null || true
    done < <(
        otool -l "$dylib" |
            awk '/cmd LC_RPATH/ { in_rpath = 1; next } in_rpath && /path / { print $2; in_rpath = 0 }'
    )
}

patch_bundled_dylibs() {
    echo "Rewriting bundled dylibs to use @loader_path"

    shopt -s nullglob
    local dylib
    for dylib in "$DEST_DIR"/*.dylib; do
        patch_dylib "$dylib"
    done
    shopt -u nullglob
}

require_tool otool
require_tool install_name_tool

if [[ ! -d "$SOURCE_DIR" ]]; then
    echo "error: source directory does not exist: $SOURCE_DIR" >&2
    exit 1
fi

copy_existing_dylibs
copy_mathapp_dylib
patch_bundled_dylibs

echo "Update process complete."
