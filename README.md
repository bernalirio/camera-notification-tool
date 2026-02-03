# Linux Camera Monitor

A lightweight, zero-overhead Rust utility that triggers a desktop notification whenever your webcam is activated or deactivated.

### Why use this?
This tool was originally built for **Arch Linux** that was lacking a built-in "Camera On" indicator.

However, because it uses the standard `notify-rust` crate, **it works out-of-the-box on virtually any Linux distribution** and any Desktop Environment that supports the standard notification protocol.

## Features
- **Instant Notification:** Reacts immediately to hardware changes via `udev`.
- **Burst Protection:** Intelligently filters out the duplicate signals and metadata events often sent by modern webcam drivers.
- **Zero-Overhead:** Idle CPU usage is effectively 0%. It sleeps until the kernel wakes it up.
- **Universal Compatibility:** Works with any notification server (`dunst`, `mako`, GNOME, KDE, etc.).

## Installation

### 1. Build the project
```bash
cargo build --release
```
### 2. Install the binary
```bash
mkdir -p ~/.local/bin
cp target/release/camera-monitor ~/.local/bin/
```

### 3. Setup Auto-Start (Optional)
To have the monitor run silently in the background when you log in:
```bash
# Create the directory if it doesn't exist
mkdir -p ~/.config/systemd/user/

# Copy the provided service file
cp systemd/camera-monitor.service ~/.config/systemd/user/

# Reload systemd and enable the service
systemctl --user daemon-reload
systemctl --user enable --now camera-monitor
```
