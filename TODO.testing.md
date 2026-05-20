# Testing Plan — Timer CLI

Ensure behavioral stability across future releases by adding comprehensive tests.

**Current state:** 11 tests in 3 files (`timer.rs`, `time.rs`, `opts.rs`)

## Safety Rules

- `cargo test` must pass after every checkbox
- Each phase is independently verifiable
- Phases 0-3 require no hardware (no audio, no display, no special devices)
- Phase 4 requires a built binary but uses `--silence` to avoid audio

---

## Phase 0: Add dev-dependencies

- [ ] Add to `Cargo.toml` `[dev-dependencies]`:
  - `assert_cmd = "2"` — CLI binary testing
  - `insta = "1"` — snapshot testing
  - `predicates = "3"` — composable assertions
- [ ] `cargo test` passes (existing 11 tests unchanged)

---

## Phase 1: Pure function tests (no new deps needed)

### 1.1 `Time::format()` — `src/time.rs`

- [ ] `0h 0m 0s` → `"0h 0m 0s"`
- [ ] `1h 0m 0s` → `"1h 0m 0s"`
- [ ] `0h 30m 0s` → `"0h 30m 0s"`
- [ ] `0h 0m 45s` → `"0h 0m 45s"`
- [ ] `2h 10m 5s` → `"2h 10m 5s"`
- [ ] `99h 59m 59s` → `"99h 59m 59s"`

### 1.2 `Time::format_ruled()` — `src/time.rs`

- [ ] With no omission flags → full format
- [ ] Omit seconds → `"2h 10m"`
- [ ] Omit minutes → `"2h"`
- [ ] Omit seconds and minutes → minimal output

### 1.3 `stopwatch::handle_key()` — `src/stopwatch.rs`

- [ ] `Space` → `Action::Pause`
- [ ] `p` → `Action::Pause`
- [ ] `l` → `Action::Lap`
- [ ] `Enter` → `Action::Lap`
- [ ] `r` → `Action::Reset`
- [ ] `q` → `Action::Quit`
- [ ] `Ctrl+C` → `Action::Quit`
- [ ] Unknown key → `Action::Noop`

### 1.4 `State` transitions — `src/stopwatch.rs`

- [ ] `State::Running` → `is_running()` returns `true`
- [ ] `State::Paused` → `is_running()` returns `false`
- [ ] `Running::toggle_pause()` → `Paused` (accumulated preserved)
- [ ] `Paused::toggle_pause()` → `Running` (accumulated preserved)
- [ ] `reset()` → fresh `Running` state

### 1.5 `ui::format_laps()` — `src/ui.rs`

- [ ] Empty laps → empty string
- [ ] Single lap → formatted string with lap number and time
- [ ] Multiple laps → numbered list with times

### 1.6 `parse_counter_time()` edge cases — `src/timer.rs`

- [ ] `"0s"` → `Some(Duration::ZERO)`
- [ ] `""` → `None`
- [ ] `"abc"` → `None`
- [ ] `"999999h"` → `Some(...)` (verify no panic)
- [ ] `"1h1h"` → `None` or handled gracefully
- [ ] `"10"` (bare number) → `Some(10s)`

### 1.7 `parse_end_time()` edge cases — `src/timer.rs`

- [ ] `"00:00"` → parses correctly
- [ ] `"23:59:59"` → parses correctly
- [ ] `"25:00"` → `None` (invalid hour)
- [ ] `"12:60"` → `None` (invalid minute)
- [ ] `"abc"` → `None`

- [ ] **Verify:** `cargo test` passes, test count increased from 11

---

## Phase 2: Snapshot tests (needs `insta` from Phase 0)

### 2.1 `Figlet::convert()` — `src/figlet/mod.rs`

- [ ] Snapshot: digits `"0"` through `"9"`
- [ ] Snapshot: `":00:00"` (colon + seconds fragment)
- [ ] Snapshot: `"1h 30m 5s"` (full timer string)

### 2.2 `Time::render()` — `src/time.rs`

- [ ] Snapshot: `120x30` terminal → full format rendered
- [ ] Snapshot: `60x20` terminal → degraded format (no seconds)
- [ ] Snapshot: `20x10` terminal → minimal or plain text fallback

### 2.3 `Time::try_render()` — `src/time.rs`

- [ ] Very small size (e.g. `10x5`) → returns `None`
- [ ] Verify degradation logic: each step omits more parts

- [ ] **Verify:** `cargo insta review` to accept snapshots, then `cargo test` passes

---

## Phase 3: CLI parsing tests — `src/opts.rs`

### 3.1 Valid invocations (`Opts::try_parse_from()`)

- [ ] `timer 5m` → `Opts { time: ["5m"], silence: false, loop: false, terminal_bell: false }`
- [ ] `timer 1h30m45s` → parses time correctly
- [ ] `timer stopwatch` → `command: Some(Command::Stopwatch)`
- [ ] `timer --silence 10s` → `silence: true`
- [ ] `timer -l -s 5m` → `loop: true, silence: true`

### 3.2 Invalid invocations

- [ ] `timer foo` → returns error (not a valid duration or time)
- [ ] `timer --invalid` → returns error (unknown flag)
- [ ] `timer` (no args) → verify current behavior (may be valid with no time)

### 3.3 Default values

- [ ] `silence` defaults to `false`
- [ ] `loop` defaults to `false`
- [ ] `terminal_bell` defaults to `false`

- [ ] **Verify:** `cargo test` passes

---

## Phase 4: Binary integration tests (needs `assert_cmd` from Phase 0)

Create `tests/integration.rs`.

### 4.1 Happy paths

- [ ] `timer 1s` — exits with code 0
- [ ] `timer --silence 1s` — exits 0 without audio
- [ ] `timer -t 1s` — exits 0 (terminal bell mode)

### 4.2 Error paths

- [ ] `timer foo` — exits non-zero, stderr contains error message
- [ ] `timer --invalid-flag` — exits non-zero

### 4.3 Stopwatch

- [ ] `timer stopwatch` — starts then killed with SIGTERM, exits cleanly

### 4.4 Loop mode

- [ ] `timer --loop --silence 1s` — runs at least one cycle, then killed

- [ ] **Verify:** `cargo test` passes

---

## Phase 5: CI integration

- [ ] Check if GitHub Actions workflow exists and includes `cargo test`
- [ ] Add `cargo test` step to CI pipeline if missing
- [ ] Verify CI passes on push

---

## Summary

| Phase | New tests (est.) | New dev-deps | Risk |
|-------|-----------------|--------------|------|
| 0     | 0               | 3            | None |
| 1     | ~30             | 0            | Very low |
| 2     | ~10             | 0 (insta)    | Low |
| 3     | ~12             | 0            | Low |
| 4     | ~6              | 0 (assert_cmd)| Low |
| 5     | 0               | 0            | Low |
| **Total** | **~58**     | **3**        | — |
