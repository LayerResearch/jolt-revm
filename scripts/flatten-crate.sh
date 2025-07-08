#!/bin/bash
# flatten-crate.sh - Flatten workspace inheritance in specific crates
# 
# This script resolves workspace inheritance (edition.workspace = true, etc.)
# by generating .crate files and copying the resolved Cargo.toml back to 
# the original crate directories.

set -e

# Global variables
MANIFEST_PATH=""
PACKAGES=()
TEMP_DIR=""

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --manifest-path)
                MANIFEST_PATH="$2"
                shift 2
                ;;
            --package)
                PACKAGES+=("$2")
                shift 2
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                echo "Error: Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    if [[ -z "$MANIFEST_PATH" ]]; then
        echo "Error: --manifest-path is required"
        show_help
        exit 1
    fi
    
    if [[ ${#PACKAGES[@]} -eq 0 ]]; then
        echo "Error: At least one --package is required"
        show_help
        exit 1
    fi
}

show_help() {
    echo "Usage: $0 --manifest-path <path> --package <name> [--package <name>...]"
    echo ""
    echo "Flatten workspace inheritance in specific crates by resolving inherited"
    echo "properties and copying the resolved Cargo.toml back to the crate."
    echo ""
    echo "Examples:"
    echo "  $0 --manifest-path overrides/revm/Cargo.toml --package revm"
    echo "  $0 --manifest-path overrides/revm/Cargo.toml --package revm --package revm-statetest-types"
}

# Setup and cleanup
setup_temp_dir() {
    TEMP_DIR=$(mktemp -d)
    echo "Using temp directory: $TEMP_DIR"
    trap cleanup_temp_dir EXIT
}

cleanup_temp_dir() {
    if [[ -n "$TEMP_DIR" && -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
        echo "✓ Cleaned up temp directory"
    fi
}

# Generate .crate file for specific package
generate_crate() {
    local manifest_path="$1"
    local package_name="$2"
    local workspace_dir="$(dirname "$manifest_path")"
    
    echo "Generating .crate file for $package_name..."
    cd "$workspace_dir"
    
    cargo +nightly publish \
        --dry-run \
        --allow-dirty \
        --no-verify \
        --package "$package_name" \
        --target-dir "$TEMP_DIR/target" \
        --manifest-path "$manifest_path" \
        -Zpackage-workspace
    
    echo "✓ Generated .crate file for $package_name"
}

# Find crate directory in workspace
find_crate_dir() {
    local workspace_dir="$1"
    local package_name="$2"
    
    # Find Cargo.toml file for this package
    find "$workspace_dir" -name "Cargo.toml" -exec grep -l "name = \"$package_name\"" {} \; | head -1
}

# Extract and copy Cargo.toml for specific package
extract_and_copy() {
    local manifest_path="$1"
    local package_name="$2"
    local workspace_dir="$(dirname "$manifest_path")"
    
    echo "Processing $package_name..."
    
    # Find the .crate file
    local crate_file=$(find "$TEMP_DIR/target/package" -name "${package_name}-*.crate" | head -1)
    
    if [[ -z "$crate_file" ]]; then
        echo "Error: .crate file not found for $package_name"
        return 1
    fi
    
    # Extract .crate file (gzipped tar)
    tar -xzf "$crate_file" -C "$TEMP_DIR/target/package/"
    
    # Find extracted directory
    local extracted_dir=$(find "$TEMP_DIR/target/package" -maxdepth 1 -name "${package_name}-*" -type d | head -1)
    
    if [[ -z "$extracted_dir" ]]; then
        echo "Error: Could not find extracted directory for $package_name"
        return 1
    fi
    
    # Find target Cargo.toml in workspace
    local target_cargo_toml=$(find_crate_dir "$workspace_dir" "$package_name")
    
    if [[ -z "$target_cargo_toml" ]]; then
        echo "Error: Could not find Cargo.toml for package $package_name in workspace"
        return 1
    fi
    
    # Copy resolved Cargo.toml
    cp "$extracted_dir/Cargo.toml" "$target_cargo_toml"
    echo "✓ Updated $target_cargo_toml"
}

# Main execution
main() {
    parse_args "$@"
    
    # Validate manifest path exists
    if [[ ! -f "$MANIFEST_PATH" ]]; then
        echo "Error: Manifest file not found: $MANIFEST_PATH"
        exit 1
    fi
    
    # Convert to absolute path
    MANIFEST_PATH="$(realpath "$MANIFEST_PATH")"
    
    echo "Flattening crate packages: ${PACKAGES[*]}"
    echo "Workspace: $MANIFEST_PATH"
    
    setup_temp_dir
    
    # Process each package
    for package in "${PACKAGES[@]}"; do
        generate_crate "$MANIFEST_PATH" "$package"
        extract_and_copy "$MANIFEST_PATH" "$package"
    done
    
    echo "✓ Crate flattening complete!"
}

# Execute main function
main "$@" 