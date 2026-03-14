# gpu-observer-hyprland

A terminal-based GPU/CPU/RAM monitor that runs in a floating kitty window in Hyprland. Toggle it on and off with a keybind — it stays alive in the background when hidden.

Displays: GPU temperature (large), GPU/memory bandwidth utilisation, power draw, clock speeds, fan speed, VRAM usage, system RAM breakdown, per-core CPU utilisation bars, utilisation history sparklines, and a live table of processes using the GPU.

Built with ratatui, nvml-wrapper (NVIDIA only), and sysinfo.

## Demo

<img src="https://github.com/krshrimali/gpu-observer-hyprland/blob/main/sample.jpeg"/>

## Build & install

```sh
cargo build --release
cp target/release/gpu-observer ~/.local/bin/
```

Make sure `~/.local/bin` is in your `$PATH`.

## Hyprland setup

Add the following to your `hyprland.conf`:

```ini
# Auto-spawn gpu-observer in kitty the first time the workspace is toggled.
# Change the path if your binary is elsewhere.
workspace = special:gpu-observer, on-created-empty:kitty --class gpu-observer --title "GPU Observer" -e gpu-observer

windowrule {
    name = gpu-observer-float
    match:class = gpu-observer

    float = yes
    size = 1200 750
    center = yes
    rounding = 16
    opacity = 0.92 override 0.92
}

# Super+G to toggle. Change the key if you prefer something else.
bind = $mainMod, G, togglespecialworkspace, gpu-observer
```

After reloading the config (`hyprctl reload`), press `Super+G` to open the panel. Press it again to hide it. The process keeps running in the background so it reappears instantly.

Press `q` or `Esc` inside the panel to quit it entirely.

## Notes

- Requires an NVIDIA GPU with the driver's NVML library (`libnvidia-ml.so`), provided by `nvidia-utils` on Arch.
- The "cached" RAM figure is an approximation (`total - used - free`) since sysinfo does not expose the kernel's page cache directly.
- Kitty's background opacity + Hyprland's blur decoration give the glassmorphism effect for free — no extra config needed beyond what's above.
