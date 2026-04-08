#!/bin/bash
# SystemVision Dependency Installation Script
# This script installs the required system libraries for building SystemVision

set -e

echo "SystemVision - Dependency Installation Script"
echo "=============================================="
echo ""

# Detect the Linux distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
    VER=$VERSION_ID
else
    echo "Cannot detect Linux distribution"
    exit 1
fi

echo "Detected OS: $OS"
echo ""

case $OS in
    ubuntu|debian|pop|linuxmint)
        echo "Installing dependencies for Ubuntu/Debian..."
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            pkg-config \
            libgtk-3-dev \
            libglib2.0-dev \
            libcairo2-dev \
            libpango1.0-dev \
            libgdk-pixbuf2.0-dev \
            libatk1.0-dev \
            libsqlite3-dev \
            libssl-dev \
            cmake \
            git
        echo "✓ Dependencies installed successfully!"
        ;;
    
    fedora|rhel|centos)
        echo "Installing dependencies for Fedora/RHEL/CentOS..."
        sudo dnf install -y \
            gcc \
            gcc-c++ \
            pkg-config \
            gtk3-devel \
            glib2-devel \
            cairo-devel \
            pango-devel \
            gdk-pixbuf2-devel \
            atk-devel \
            sqlite-devel \
            openssl-devel \
            cmake \
            git
        echo "✓ Dependencies installed successfully!"
        ;;
    
    arch|manjaro)
        echo "Installing dependencies for Arch Linux..."
        sudo pacman -S --needed \
            base-devel \
            pkg-config \
            gtk3 \
            glib2 \
            cairo \
            pango \
            gdk-pixbuf2 \
            atk \
            sqlite \
            openssl \
            cmake \
            git
        echo "✓ Dependencies installed successfully!"
        ;;
    
    opensuse*|sles)
        echo "Installing dependencies for openSUSE..."
        sudo zypper install -y \
            gcc \
            gcc-c++ \
            pkg-config \
            gtk3-devel \
            glib2-devel \
            cairo-devel \
            pango-devel \
            gdk-pixbuf-devel \
            atk-devel \
            sqlite3-devel \
            libopenssl-devel \
            cmake \
            git
        echo "✓ Dependencies installed successfully!"
        ;;
    
    *)
        echo "Unsupported distribution: $OS"
        echo ""
        echo "Please install the following packages manually:"
        echo "  - build-essential / base-devel"
        echo "  - pkg-config"
        echo "  - GTK3 development libraries"
        echo "  - GLib2 development libraries"
        echo "  - Cairo development libraries"
        echo "  - Pango development libraries"
        echo "  - GDK-Pixbuf development libraries"
        echo "  - ATK development libraries"
        echo "  - SQLite3 development libraries"
        echo "  - OpenSSL development libraries"
        echo "  - CMake"
        echo "  - Git"
        exit 1
        ;;
esac

echo ""
echo "Verifying installation..."

# Verify pkg-config can find the libraries
MISSING=0

check_lib() {
    if pkg-config --exists "$1" 2>/dev/null; then
        echo "✓ $1 found (version $(pkg-config --modversion $1))"
    else
        echo "✗ $1 NOT FOUND"
        MISSING=1
    fi
}

check_lib "gtk+-3.0"
check_lib "glib-2.0"
check_lib "cairo"
check_lib "pango"
check_lib "gdk-pixbuf-2.0"
check_lib "atk"
check_lib "sqlite3"

echo ""

if [ $MISSING -eq 0 ]; then
    echo "✓ All dependencies are installed correctly!"
    echo ""
    echo "You can now build SystemVision with:"
    echo "  cargo build"
    echo ""
    echo "Or run it directly with:"
    echo "  cargo run"
else
    echo "✗ Some dependencies are missing. Please install them manually."
    exit 1
fi

# Made with Bob
