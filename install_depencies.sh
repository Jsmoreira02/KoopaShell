#!/bin/bash

# Function to check the Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
    elif [ -f /etc/redhat-release ]; then
        DISTRO="rhel"
    elif [ -f /etc/debian_version ]; then
        DISTRO="debian"
    else
        DISTRO=$(uname -s)
    fi
}

# Function for installing packages on Debian/Ubuntu-based distributions
install_debian() {
    sudo apt-get update
    sudo apt-get install -y cmake g++ gcc zlib1g-dev libx11-dev libxext-dev libxrender-dev libxrandr-dev libxinerama-dev libxcursor-dev libxfixes-dev libx11-xcb-dev libxss-dev libxdmcp-dev libpng-dev pkg-config
}

# Function for installing packages on Fedora/RHEL-based distributions
install_rhel() {
    sudo dnf install -y cmake gcc-c++ gcc zlib-devel libX11-devel libXext-devel libXrender-devel libXrandr-devel libXinerama-devel libXcursor-devel libXfixes-devel libXdmcp-devel libXss-devel libpng-devel pkg-config
}

# Function for installing packages on Arch Linux-based distributions
install_arch() {
    sudo pacman -Sy --needed cmake gcc gcc-libs zlib libx11 libxext libxrender libxrandr libxinerama libxcursor libxfixes libxdmcp libxss libpng pkgconf
}

# Detects the Linux distribution
detect_distro

# Executes the appropriate installation command based on the distribution detected
case $DISTRO in
    ubuntu|debian|kali)
        echo "Detected Debian/Ubuntu based system."
        install_debian
        ;;
    fedora|rhel|centos)
        echo "Detected Fedora/RHEL/CentOS based system."
        install_rhel
        ;;
    arch)
        echo "Detected Arch Linux based system."
        install_arch
        ;;
    *)
        echo "Unsupported distribution: $DISTRO" >&2
        exit 1
        ;;
esac

echo "Installation completed."

# Install KoopaShell
if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is not installed. Please install Rust and Cargo before proceeding." >&2
    exit 1
fi

cargo install --git https://github.com/Jsmoreira02/KoopaShell.git
