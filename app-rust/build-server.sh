#!/bin/bash

echo "Building Rust server application..."

# Build the application in release mode
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Server binary: target/release/axum_mysql_app"
    echo ""
    echo "To run the server:"
    echo "  ./target/release/axum_mysql_app server"
    echo "  or simply:"
    echo "  ./target/release/axum_mysql_app"
else
    echo "❌ Build failed!"
    exit 1
fi
