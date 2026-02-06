# Linux Camera Monitor
This tool was originally built for **Arch Linux** that was lacking a built-in "Camera On" indicator.

However, because it uses the standard `notify-rust` crate, **it works out-of-the-box on virtually any Linux distribution** and any Desktop Environment that supports the standard notification protocol.

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

## Licence
MIT License
