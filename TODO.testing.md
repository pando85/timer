# Testing Plan — Hardware & Concurrency Layer

Cover the untested sound/beep/countdown layer that caused the 0.11.3 regression.

**Current state:** 63 tests covering pure logic, CLI parsing, rendering, and binary exit codes.

**Goal:** Make `countdown()` completion logic (parallel sound+beep, silence mode, terminal bell)
and `play_beep()`/`play_sound()` orchestration testable without hardware.

## Safety Rules

- `cargo test` must pass after every checkbox
- Each phase is independently verifiable
- No behavior changes to production code — only refactoring for testability

---

## Phase 1: Extract alert traits

Introduce trait abstractions so `countdown()` doesn't depend on concrete hardware.

### 1.1 Create `src/alert.rs`

- [ ] Define `Alert` trait with `fn play(&self) -> Result<()>`
- [ ] Implement `BeepAlert` struct wrapping existing `beep::beep()` logic from `timer::play_beep()`
- [ ] Implement `SoundAlert` struct wrapping existing `Sound::new()` + `play()` from `timer::play_sound()`
- [ ] Implement `SilentAlert` struct (no-op, for `--silence` mode)
- [ ] Implement `TerminalBellAlert` struct (prints `\x07`)
- [ ] Unit test: `SilentAlert::play()` returns `Ok(())`
- [ ] Unit test: `TerminalBellAlert` writes bell character
- [ ] Register module in `main.rs`
- [ ] **Verify:** `cargo test` passes, no behavior change

### 1.2 Refactor `countdown()` to use traits

- [ ] Change `countdown()` signature to accept `&dyn Alert` or generic `A: Alert` for beep and sound
- [ ] Replace direct `play_beep()` / `play_sound()` calls with trait methods
- [ ] Replace `std::thread::spawn` with a pluggable concurrency strategy (or keep spawn but pass closures)
- [ ] Keep existing behavior — call sites in `main.rs` construct concrete types
- [ ] **Verify:** `cargo test` passes, `timer 1s --silence` still works manually

---

## Phase 2: Test countdown completion logic

Test the alert orchestration that was broken in 0.11.2 (sequential vs parallel).

### 2.1 Countdown alert tests

- [ ] Test silence mode: no alerts fired when `--silence` is set
- [ ] Test terminal bell: bell character written when `--terminal-bell` is set
- [ ] Test beep-only path: `BeepAlert::play()` called correct number of times
- [ ] Test parallel execution: both `BeepAlert` and `SoundAlert` are invoked (not sequential)
- [ ] Test thread panic: `SoundAlert` thread panic is caught and returns error
- [ ] **Verify:** `cargo test` passes

### 2.2 `play_beep()` orchestration tests

Using mock `Alert` implementations that record calls:

- [ ] Verify `BEEP_REPETITIONS` (5) calls to `play()`
- [ ] Verify fallback: when beep fails, sleep replaces it (no panic)
- [ ] **Verify:** `cargo test` passes

---

## Phase 3: Test `resize_term()` and `parse_time()`

### 3.1 `resize_term()` — `src/timer.rs`

- [ ] Test positive counter: draws remaining time
- [ ] Test zero/negative counter: draws `Duration::ZERO`
- [ ] Both need a mock writer (`Vec<u8>`) — already generic over `W: io::Write`
- [ ] **Verify:** `cargo test` passes

### 3.2 `parse_time()` — `src/main.rs`

- [ ] Test duration input (`"5m"`) returns a future `OffsetDateTime`
- [ ] Test target time input (`"12:00"`) returns today's or tomorrow's date
- [ ] Test invalid input returns `None`
- [ ] Note: function uses `OffsetDateTime::now_utc()` — may need `#[cfg(test)]` time injection
- [ ] **Verify:** `cargo test` passes

---

## Phase 4: Edge-case and regression tests

- [ ] Test `BEEP_FREQ` constant hasn't changed (440 Hz)
- [ ] Test `BEEP_REPETITIONS` constant hasn't changed (5)
- [ ] Test `SOUND_START_DELAY` < `BEEP_DELAY` (timing invariant for parallel play)
- [ ] Regression: test that countdown with both beep and sound uses threading (not sequential)
- [ ] **Verify:** `cargo test` passes

---

## Not in scope (requires hardware or unsafe mocking)

| Function | Reason |
|----------|--------|
| `beep::driver_evdev()` | Unsafe FFI, needs real device |
| `beep::driver_console()` | Unsafe ioctl, needs real device |
| `beep::get_driver()` | Runtime hardware detection |
| `beep::get_device()` | Filesystem device discovery |
| `Sound::new()` | Requires audio device |
| Signal handling in `run_countdown()` | Complex OS signal interaction |

---

## Summary

| Phase | New tests (est.) | Refactoring needed | Risk |
|-------|-----------------|-------------------|------|
| 1     | 2               | New `alert.rs` module | Low |
| 2     | 6               | Trait-based countdown | Medium |
| 3     | 5               | Minor (time injection) | Low |
| 4     | 4               | None | Very low |
| **Total** | **~17**      | 1 new module + trait refactor | — |
