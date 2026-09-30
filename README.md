<h1 align="center">
  <br>
  <img src="https://raw.githubusercontent.com/pando85/timer/master/assets/logo.svg" alt="Timer logo" width="200">
  <br>
  Timer
  <br>
  <br>
</h1>

<p align="center">
  <strong>A simple terminal countdown timer and stopwatch with a large adaptive display and audible alarms.</strong>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/pando85/timer/master/assets/demo.gif" alt="Timer demo">
</p>

![Build status](https://img.shields.io/github/actions/workflow/status/pando85/timer/rust.yml?branch=master)
![Timer license](https://img.shields.io/github/license/pando85/timer)

Timer is designed to stay out of the way: no configuration, no daemon, and no workflow to learn. Give it a duration or a target clock time and leave it visible in a terminal, tab, or tmux pane.

```bash
timer 25m          # countdown for 25 minutes
timer 14:30        # alarm at 14:30
timer --loop 5m    # repeat a countdown
timer stopwatch    # count up from zero
```

## Features

- Flexible duration input: `10`, `30s`, `15min`, `1h30m`, `1h 30m`
- Target clock times such as `08:25` or `14:30`
- Large centered display that adapts to the terminal size
- Remaining time shown in the terminal title
- Built-in sound alarm
- Linux PC-speaker support
- Optional terminal bell, useful with visual-bell terminal configurations
- Repeating countdowns with `--loop`
- Interactive stopwatch with pause, lap, and reset controls

## Usage

### Countdown

Durations can be written in several forms:

```bash
timer 10
timer 30s
timer 15min
timer 1h30m
timer 1h 30m
```

A bare number is interpreted as seconds.

You can also specify the clock time at which the alarm should fire:

```bash
timer 08:25
timer 14:30
timer 18:45:30
```

If the requested clock time has already passed, Timer schedules it for the next day.

To repeat a countdown indefinitely:

```bash
timer --loop 25m
```

To suppress audible alarms:

```bash
timer --silence 10m
```

To send a terminal bell when the timer finishes:

```bash
timer --terminal-bell 10m
```

Flags can be combined. For example, this produces only the terminal bell:

```bash
timer --silence --terminal-bell 11:00
```

### Stopwatch

Start the stopwatch with:

```bash
timer stopwatch
```

Controls:

- `Space` or `p` — pause/resume
- `l` or `Enter` — record a lap
- `r` — reset
- `q` or `Ctrl+C` — quit

## Installation

### Cargo

Install from crates.io:

```bash
cargo install timer-cli
```

### Arch Linux

Install from the AUR:

```bash
yay -S timer-rs
```

or install the prebuilt AUR package:

```bash
yay -S timer-rs-bin
```

### Prebuilt binaries

Prebuilt binaries for supported Linux and macOS targets are attached to every [GitHub release](https://github.com/pando85/timer/releases).

For example, on x86_64 Linux you can download and install the latest release with:

```bash
url=$(curl -s https://api.github.com/repos/pando85/timer/releases/latest \
  | grep browser_download_url \
  | grep 'x86_64-unknown-linux-gnu.tar.gz"' \
  | cut -d '"' -f 4)

curl -L "$url" | tar xz
sudo install timer /usr/local/bin/timer
```

Release assets also include SHA-256 checksums.

## Alarms

By default, Timer plays its bundled alarm sound when the countdown finishes. On Linux it can also use the built-in PC speaker when available.

### PC speaker

To use the built-in case speaker, load either the `pcspkr` kernel module (recommended) or `snd-pcsp`.

Access to the speaker device may require additional permissions. See [`PERMISSIONS.md`](PERMISSIONS.md) for the recommended setup. It avoids running Timer as root or granting broad access to input devices.

### Terminal bell

With `-t` / `--terminal-bell`, Timer also emits the terminal bell character (`\a`). This is particularly useful when your terminal is configured to use a visual bell.

```bash
timer -t -s 11:00
```

## Why Timer?

Timer deliberately keeps its scope small. It is meant to be a fast, native terminal utility rather than a task manager or productivity suite: start it with a single command, keep the countdown visible, and get an alarm when it reaches zero.
