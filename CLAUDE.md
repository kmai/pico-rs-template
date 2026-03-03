# CLAUDE.md — RP2040 project context

## Hardware

**Board:** Clone Raspberry Pi Pico (RP2040)
**RP2040 revision:** B2
**Crystal:** 12 MHz external (standard)

### Flash chip
- **Manufacturer:** Puya Semiconductor
- **Part:** P25Q16H
- **Flash ID:** `0xE6626005A7579837` (confirmed via `picotool info -a`)
- **Capacity:** 2048K
- **Compatible boot2:** `BOOT_LOADER_W25Q080` (quad-SPI, full speed)

---

## Rules for this project

### boot2
Always declare boot2 explicitly in `src/main.rs`. Do not rely on the BSP default:

```rust
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
```

### memory.x
Always include the explicit `SECTIONS` block in `memory.x` to place `.boot2` at
`0x10000000`. Without it the board reboots in a loop after flashing. See `memory.x`.

After building, verify with:

```bash
arm-none-eabi-objdump -h target/thumbv6m-none-eabi/release/<bin> | grep boot2
# Must show: 10000000
```

### critical-section
Never add `critical-section-single-core` to `cortex-m` features. `rp2040-hal` provides
its own implementation via `critical-section-impl` and the two conflict at compile time.

### defmt
Do not add `-C link-arg=-Tdefmt.x` to `.cargo/config.toml` unless `defmt` is an actual
dependency. It will cause a linker error.

---

## Standard Cargo.toml dependencies

```toml
[dependencies]
rp2040-hal   = { version = "0.10", features = ["rt", "critical-section-impl"] }
rp2040-boot2 = "0.3"
cortex-m-rt  = "0.7"
cortex-m     = "0.7"           # no features
embedded-hal = "1.0"
panic-halt   = "0.2"
```

---

## Standard imports

```rust
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::Pins,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
```

---

## Toolchain

- **Target:** `thumbv6m-none-eabi`
- **Flash tool:** `elf2uf2-rs -d`
- **Rust edition:** 2021

### `.cargo/config.toml`

```toml
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
runner = "elf2uf2-rs -d"
rustflags = [
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
]
```

---

## Common errors and fixes

| Error | Cause | Fix |
|---|---|---|
| Board remounts after flashing | Wrong/missing boot2 or bad `.boot2` address | Use `BOOT_LOADER_W25Q080` + `SECTIONS` block in `memory.x` |
| `.boot2` not at `0x10000000` | Linker script ordering | Add `SECTIONS` block to `memory.x` |
| `RawRestoreStateInner` defined multiple times | Two critical-section impls | Remove `critical-section-single-core` from `cortex-m` features |
| `cannot find linker script defmt.x` | defmt flag without defmt dep | Remove `-C link-arg=-Tdefmt.x` from `.cargo/config.toml` |
| `no method named freq` | `Clock` trait not in scope | Add `Clock` to `clocks::` import |
| `no method named set_high/set_low` | `OutputPin` trait not in scope | Add `use embedded_hal::digital::OutputPin` |


