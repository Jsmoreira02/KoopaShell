![Untitled_design_1_-removebg-preview](https://github.com/user-attachments/assets/92ab35e6-ef63-4a9d-b0a5-93f4fcdbb60f)

#

<div>
    <img src="https://img.shields.io/badge/Language%20-Rust-darkorange.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Target OS%20-Linux, Windows-darkblue.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Cargo builds%20-clippers, rustyline, clap-beige.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/License%20-CC BY_ND 4.0-lightgreen.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Type%20-C2 Like | Multi Handler-purple.svg" style="max-width: 100%;">
    
</div></br>


| :exclamation:  **Under active development**  :exclamation: |

> C2, also known as Command and Control, refers to the infrastructure and techniques used by hackers to maintain control over compromised systems or networks. By establishing a connection with these compromised systems, hackers can execute their malicious activities

## About:

Koopa Shell is an advanced tool developed in Rust, designed as a Multiple Reverse TCP Shell Handler and Stage 0/1 C2 Framework, it elevates shell interactions by generating obfuscated PowerShell payload, ensuring both stealth and efficiency. Koopa Shell supports seamless connections across Linux and Windows environments, making it a versatile for working in diverse infrastructures.

- Support for multiple reverse TCP connections.
- Make quick and easy transitions between all your reverse shell connections
- Compatible with Linux and Windows systems.
- Improved functionality for controlling and using shell sessions.
- Generation of obfuscated powershell payloads to avoid detection.

Made for pentest operations or attack simulations focused on evading initial detection and managing sessions in different environments.


| **New features and implementations will be continually added to the project** |

## New Features:

- 📌**NEW**: Added an easier way to connect to and navigate through sessions (Sessions index)
- 📌**NEW**: The format of the session IDs has been changed to hexadecimal (16 bit) code.
- 🕒 **Coming soon**: New payload types for linux and windows

## Usage:

### Bypass Windows AV:

https://github.com/user-attachments/assets/2ec6d9dc-d92e-4c1f-bfe7-a44ac5cb11aa


### Multi sessions
https://github.com/user-attachments/assets/930dcd80-e409-4a02-aedb-8dbe44472945

##

## Installation:

```
  git clone https://github.com/Jsmoreira02/KoopaShell.git
  cd KoopaShell
  chmod +x install_dependencies.sh
  bash install_dependecies.sh
```

or

```bash
  curl -o install_dependecies.sh https://raw.githubusercontent.com/Jsmoreira02/KoopaShell/main/install_dependecies.sh && bash install_dependecies.sh
```

## Dependecies:

### Debian/Ubuntu:
> sudo apt-get install -y cmake g++ gcc zlib1g-dev libx11-dev libxext-dev libxrender-dev libxrandr-dev libxinerama-dev libxcursor-dev libxfixes-dev libx11-xcb-dev libxss-dev libxdmcp-dev libpng-dev pkg-config

### Fedora/RHEL-based:
> sudo dnf install -y cmake gcc-c++ gcc zlib-devel libX11-devel libXext-devel libXrender-devel libXrandr-devel libXinerama-devel libXcursor-devel libXfixes-devel libXdmcp-devel libXss-devel libpng-devel pkg-config

### Arch Linux-based: 
> sudo pacman -Sy --needed cmake gcc gcc-libs zlib libx11 libxext libxrender libxrandr libxinerama libxcursor libxfixes libxdmcp libxss libpng pkgconf

## Tribute:

I'd like to take a moment to express my absolute admiration for these offensive security programmers/researchers. They inspired me to decide to create this tool, and I really hope I can achieve this level of capability.

- [@t3l3machus](https://github.com/t3l3machus)
- [@Z4nzu](https://github.com/Z4nzu)
- [@loseys](https://github.com/loseys)
- [@Teach2Breach](https://github.com/Teach2Breach)

# Warning:    
> I am not responsible for any illegal use or damage caused by this tool. It was written for fun, not evil and is intended to raise awareness about cybersecurity.


***Have a good hack :D***


