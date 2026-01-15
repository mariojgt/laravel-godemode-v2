#!/bin/bash

# Laravel God Mode - Development Setup Script

echo "🚀 Setting up Laravel God Mode..."

# Check for required tools
echo "📋 Checking requirements..."

# Check Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi
echo "✅ Node.js $(node --version)"

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first: https://rustup.rs/"
    exit 1
fi
echo "✅ Rust $(rustc --version)"

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "⚠️  Docker is not installed. You'll need Docker to run projects."
else
    echo "✅ Docker $(docker --version)"
fi

# Install npm dependencies
echo ""
echo "📦 Installing npm dependencies..."
npm install

# Setup templates directory
echo ""
echo "📁 Setting up templates..."
TEMPLATES_DIR="$HOME/.laravel-godmode/templates"
mkdir -p "$TEMPLATES_DIR"

# Copy templates if they exist
if [ -d "./templates" ]; then
    cp -r ./templates/* "$TEMPLATES_DIR/"
    echo "✅ Templates copied to $TEMPLATES_DIR"
fi

# Create projects directory
PROJECTS_DIR="$HOME/Documents/laravel-godmode/projects"
mkdir -p "$PROJECTS_DIR"
echo "✅ Projects directory: $PROJECTS_DIR"

echo ""
echo "🎉 Setup complete!"
echo ""
echo "To start development:"
echo "  npm run tauri dev"
echo ""
echo "To build for production:"
echo "  npm run tauri build"
echo ""
