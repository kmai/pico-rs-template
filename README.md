# {{project-name}}

{{description}}

Built with Rust on a clone Raspberry Pi Pico (RP2040) with Puya P25Q16H flash.

---

## Requirements

- Rust stable toolchain
- `thumbv6m-none-eabi` target
- `elf2uf2-rs` for USB flashing

### One-time setup

```bash
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs
```

---

## Building and flashing

1. Hold **BOOTSEL**, plug in USB — Pico mounts as `RPI-RP2`
2. Release BOOTSEL
3. Run:

```bash
cargo run --release
```

---

## Project structure

```
[project-name]/
├── .cargo/
│   └── config.toml     # build target + runner
├── src/
│   └── main.rs         # application entry point
├── memory.x            # linker memory layout
├── Cargo.toml
├── CLAUDE.md           # hardware context for AI assistants
└── README.md
```

---

## Hardware notes

This project targets a **clone Pico with Puya P25Q16H flash** (ID `0xE6626005`).
Two things are required that differ from a standard Pico setup:

### Explicit boot2

Declared in `src/main.rs` — do not remove:

```rust
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
```

### Explicit linker placement in memory.x

`memory.x` includes a `SECTIONS` block that forces `.boot2` to `0x10000000`.
Without it the board reboots in a loop after every flash.

To verify after building:

```bash
arm-none-eabi-objdump -h target/thumbv6m-none-eabi/release/[project-name] | grep boot2
# Address column must read: 10000000
```

---

## Identifying flash on a different clone

Run `picotool info -a` with the board in BOOTSEL mode:

| Flash ID prefix | Chip | boot2 to use |
|---|---|---|
| `0xE662` | Puya P25Q16H | `BOOT_LOADER_W25Q080` |
| `0xEF40` | Winbond W25Q16 | `BOOT_LOADER_W25Q080` |
| `0xC840` | GigaDevice GD25Q16 | `BOOT_LOADER_GD25Q64CS` |
| Unknown | Any | `BOOT_LOADER_GENERIC_03H` (slower, universal) |

---

## Dependencies

```toml
rp2040-hal   = { version = "0.10", features = ["rt", "critical-section-impl"] }
rp2040-boot2 = "0.3"
cortex-m-rt  = "0.7"
cortex-m     = "0.7"
embedded-hal = "1.0"
panic-halt   = "0.2"
```

> Do not add `critical-section-single-core` to `cortex-m` — it conflicts with
> `rp2040-hal`'s own critical section implementation.
