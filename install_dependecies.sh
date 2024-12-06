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
    sudo apt-get install -y build-essential pkg-config libssl-dev xclip xsel libx11-dev xorg-dev libxcb-composite0-dev libxext-dev 
}

# Function for installing packages on Fedora/RHEL-based distributions
install_rhel() {
    sudo dnf groupinstall "Development Tools"
    sudo dnf install -y cmake gcc-c++ gcc xclip xsel pkg-config ncurses-devel

}

# Function for installing packages on Arch Linux-based distributions
install_arch() {
    sudo pacman -S --needed --noconfirm base-devel gcc openssl pkgconf libxcb libx11
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
