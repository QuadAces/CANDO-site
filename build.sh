#!/bin/bash
set -e

# Install dioxus-cli
cargo install dioxus-cli

# Build for web
dioxus build --release --target web