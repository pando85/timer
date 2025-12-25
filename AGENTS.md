# AGENTS.md - Timer CLI Codebase Guide

## Project Overview

**Timer CLI** is a Rust-based terminal countdown timer and stopwatch with ASCII art display, sound alerts, and hardware PC speaker support. It provides a simple, elegant way to set countdown timers or run a stopwatch with large, centered FIGlet text output.

**Repository:** https://github.com/pando85/timer
**Binary name:** `timer`
**Package name:** `timer-cli`

## Architecture

```
src/
├── main.rs          # Entry point, CLI parsing, signal handling, main loop
├── opts.rs          # CLI options structure (clap derive)
├── timer.rs         # Core countdown logic, time string parsing
├── stopwatch.rs     # Core stopwatch logic, state machine, keyboard handling
├── time.rs          # Time struct, formatting, FIGlet ASCII rendering
├── ui.rs            # Terminal UI (crossterm): setup, draw, restore
├── beep.rs          # Linux PC speaker hardware beep (ioctl)
├── constants.rs     # Timing constants for beep/sound
├── utils.rs         # Thread utilities with timeout
├── figlet/
│   ├── mod.rs       # FIGlet ASCII art text conversion
│   └── univers.flf  # FIGlet font file (embedded)
└── sound/
    ├── mod.rs       # Audio playback via rodio
    └── beep.ogg     # Embedded notification sound
```

## Module Responsibilities

### `main.rs` - Application Entry Point
- Parses CLI arguments using `clap`
- Routes to countdown or stopwatch based on subcommand
- Converts input time to `OffsetDateTime` target (for countdown)
- Sets up terminal (alternate screen, hidden cursor)
- Spawns countdown thread
- Handles Unix signals (SIGWINCH for resize, SIGTERM/SIGINT/SIGQUIT for exit)
- Supports loop mode (`--loop` flag)

**Key functions:**
- `parse_time(input_time: &str) -> Option<OffsetDateTime>` - Parses duration or target time
- `handle_countdown(w, end, opts) -> Result<()>` - Runs countdown with terminal restoration
- `run_stopwatch()` - Runs stopwatch mode
- `run_countdown(opts)` - Runs countdown mode
- `main()` - Entry point orchestrating everything

### `opts.rs` - CLI Options
Defines the `Opts` struct with clap derive:
```rust
pub struct Opts {
    pub command: Option<Command>, // Subcommand (stopwatch)
    pub r#loop: bool,             // -l, --loop: Repeat countdown
    pub silence: bool,            // -s, --silence: Disable sounds
    pub terminal_bell: bool,      // -t, --terminal-bell: Enable bell character
    pub time: Vec<String>,        // Positional: time input
}

pub enum Command {
    Stopwatch,  // Start a stopwatch (counts up from zero)
}
```

**Supported time formats (countdown):**
- Duration: `10s`, `5m`, `2h10m`, `1h30m45s`, `15min`, `10` (seconds)
- Target time: `12:00`, `08:25`, `13:45:43`

### `timer.rs` - Core Countdown Logic
- `parse_counter_time(s: &str) -> Option<Duration>` - Parses duration strings using regex
- `parse_end_time(s: &str) -> Option<OffsetDateTime>` - Parses target time strings
- `countdown(w, end, opts) -> Result<()>` - Main tail-recursive countdown loop
- `resize_term(w, end) -> Result<()>` - Redraws on terminal resize
- `play_beep() -> Result<()>` - Plays hardware PC speaker beep
- `play_sound() -> Result<()>` - Plays audio file with timeout

**Countdown algorithm:** Uses tail-call optimization (`tailcall` crate) to recursively count down, drawing the UI each second until the end time is reached.

### `stopwatch.rs` - Core Stopwatch Logic
- `run(w) -> Result<()>` - Main stopwatch loop with keyboard handling
- `State` enum - State machine (Running/Paused) tracking elapsed time
- `handle_key(key) -> Action` - Maps keyboard input to actions

**Stopwatch controls:**
- `Space` / `p` - Toggle pause/resume
- `l` / `Enter` - Record lap time
- `r` - Reset stopwatch
- `q` / `Ctrl+C` - Quit

**State machine:**
```rust
enum State {
    Running { start: Instant, accumulated: Duration },
    Paused { accumulated: Duration },
}
```

### `time.rs` - Time Representation & Rendering
Defines the `Time` struct:
```rust
pub struct Time {
    hours: u64,
    minutes: u8,
    seconds: u8,
}
```

**Key functions:**
- `From<&Duration> for Time` - Converts `time::Duration` to `Time`
- `format() -> String` - Returns formatted string like `2h 10m 5s`
- `render(size: (u16, u16)) -> String` - Renders as centered FIGlet ASCII art

**Rendering strategy (graceful degradation):**
1. Full format: `2h 10m 5s`
2. If too wide: `2h 10m` (omit seconds)
3. If still too wide: `2h` (omit minutes)
4. If still too wide: plain text (no FIGlet)

### `ui.rs` - Terminal UI Management
Uses `crossterm` for terminal operations:
- `draw(w, counter: Duration) -> Result<()>` - Clears screen, sets title, renders time
- `draw_with_laps(w, elapsed, laps, is_running) -> Result<()>` - Draws stopwatch with lap times
- `set_up_terminal(w) -> Result<()>` - Enters alternate screen, hides cursor
- `restore_terminal(w) -> Result<()>` - Shows cursor, leaves alternate screen

### `beep.rs` - Hardware PC Speaker
Linux-specific hardware beep via ioctl:
- Searches for beep devices: `/dev/input/by-path/platform-pcspkr-event-spkr`, `/dev/console`, TTYs
- Two driver strategies: Console driver (KIOCSOUND) and Evdev driver (input_event)
- `beep(freq: i32, time: Duration) -> Result<()>` - Plays beep at frequency

### `sound/mod.rs` - Audio Playback
Uses `rodio` crate:
- Embeds `beep.ogg` at compile time
- `Sound::new() -> Result<Sound>` - Opens audio stream
- `Sound::play(&self) -> Result<()>` - Plays the embedded sound

### `figlet/mod.rs` - ASCII Art Text
FIGlet font rendering:
- Embeds `univers.flf` font at compile time
- `Figlet::convert(&self, s: &str) -> String` - Converts text to ASCII art

### `constants.rs` - Timing Constants
```rust
pub const BEEP_DELAY: u64 = 100;        // ms between beeps
pub const BEEP_DURATION: u64 = 400;     // ms per beep
pub const BEEP_REPETITIONS: usize = 5;  // number of beeps
pub const BEEP_FREQ: i32 = 440;         // Hz (A4 note)
```

### `utils.rs` - Thread Utilities
- `JoinWithTimeout<T>` - Thread handle with timeout support
- `spawn_thread(f) -> JoinWithTimeout<T>` - Spawns thread with completion signal
- `join(timeout) -> Option<T>` - Waits with timeout

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `crossterm` | Cross-platform terminal manipulation |
| `time` | Date/time handling and formatting |
| `regex` | Time input string parsing |
| `rodio` | Audio playback (vorbis support) |
| `signal-hook` | Unix signal handling |
| `tailcall` | Tail-call optimization for recursion |
| `libc` / `nix` | Low-level Linux ioctl for PC speaker |
| `glob` | Finding TTY devices |

## Application Flow

### Countdown Flow
```
1. main()
   ├── Parse CLI args → Opts
   ├── parse_time(input) → OffsetDateTime
   │   ├── Try parse_counter_time() for "5m30s" style
   │   └── Or parse_end_time() for "12:30" style
   ├── ui::set_up_terminal()
   ├── Spawn countdown thread
   │   └── timer::countdown() [tail-recursive]
   │       ├── Calculate remaining = end - now
   │       ├── ui::draw(remaining)
   │       ├── sleep(1 second)
   │       └── Recurse until done, then play alerts
   └── Main thread: Signal handler loop
       ├── SIGWINCH → resize_term()
       └── SIGTERM/SIGINT/SIGQUIT → cleanup & exit
```

### Stopwatch Flow
```
1. main()
   ├── Parse CLI args → Opts
   ├── Match Command::Stopwatch
   ├── ui::set_up_terminal()
   └── stopwatch::run()
       ├── Initialize State::Running
       └── Loop:
           ├── Calculate elapsed time
           ├── ui::draw_with_laps(elapsed, laps)
           ├── Poll keyboard events (100ms timeout)
           └── Handle key → Action (Quit/Pause/Lap/Reset)
```

## Testing

Run tests with:
```bash
cargo test
```

## Building

```bash
cargo build --release
```

The binary is optimized with LTO and symbol stripping (see `Cargo.toml` profile).

## Code Style

- Uses Rust 2024 edition
- Tail-call optimization for recursive functions
- Embedded assets (font, sound) at compile time
- Error handling via `Box<dyn std::error::Error>`
- Modular separation of concerns

## Extension Points

When adding new features:
- **New CLI options:** Add to `Opts` struct in `opts.rs`
- **New time formats:** Extend parsing in `timer.rs`
- **UI changes:** Modify `ui.rs` and `time.rs` rendering
- **New alerts:** Add to `beep.rs` or `sound/mod.rs`
- **Subcommands:** Consider using clap subcommands in `opts.rs`
