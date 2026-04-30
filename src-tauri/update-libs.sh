#!/bin/bash

# Define source and destination paths
SOURCE_DIR="$HOME/mathappws/llama.cpp/build-cpu/bin"
DEST_DIR="/Users/kamran/mathappws/math-app/src-tauri/assets/libs"

# Ensure destination directory exists
mkdir -p "$DEST_DIR"

echo "Checking for dylib updates..."

# Loop through all .dylib files in the source directory
for file in "$SOURCE_DIR"/*.dylib; do
    # Get the filename without the path
    filename=$(basename "$file")
    
    # Check if the file already exists in the destination directory
    if [ -f "$DEST_DIR/$filename" ]; then
        echo "Updating $filename..."
        cp "$file" "$DEST_DIR/$filename"
    else
        echo "Skipping $filename (not present in destination)"
    fi
done

echo "Update process complete."