#!/bin/bash
set -e

# Ensure Rust bin directory is in PATH
export PATH="$HOME/.cargo/bin:$PATH:/rust/bin"

# Install dioxus-cli if not already installed
cargo install dioxus-cli || echo "dioxus-cli already installed"

# Build for web using the binary
dx build --web --release 