<h1 align="center">
  <br>
  <img src="https://raw.githubusercontent.com/pando85/timer/main/assets/logo.svg" alt="logo" width="200">
  <br>
  Timer
  <br>
  <br>
</h1>

<p align="center">
  <img src="https://raw.githubusercontent.com/pando85/timer/main/assets/demo.gif" alt="demo">
</p>

![Build status](https://img.shields.io/github/workflow/status/pando85/timer/Rust/main)
![Timer license](https://img.shields.io/github/license/pando85/timer)

Simple count down terminal alarm.

## Features

- Multiple input options.
- Play sound when finish. (Beep included!)
- Big centered output.

## Installation

### Cargo

```
cargo install timer_core
```

### Archlinux

```
yay -S timer-rs
```

### Binaries

Binaries are made available each release for the Linux and MacOS operating systems.

You can download a prebuilt binary from our [Releases](https://github.com/pando85/timer/releases).

```
curl -s https://api.github.com/repos/pando85/timer/releases/latest \
  | grep browser_download_url \
  | grep $(uname -m) \
  | grep linux \
  | cut -d '"' -f 4 \
  | xargs curl -L \
  | tar xvz
sudo mv timer /usr/local/bin
```

## Beep

If you want to enable beep from your built-in case speaker you will need to run one of these
kernel modules: `pcspkr` (recommended) or `snd-pcsp`.

In addition, to use beep as normal user read the [`PERMISSIONS.md`](PERMISSIONS.md) file.

