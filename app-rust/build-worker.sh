#!/bin/bash

echo "Building Rust worker application..."

# Build the application in release mode
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Worker binary: target/release/axum_mysql_app"
    echo ""
    echo "To run the worker:"
    echo "  ./target/release/axum_mysql_app worker"
else
    echo "❌ Build failed!"
    exit 1
fi
