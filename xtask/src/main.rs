use std::{
    env,
    process::{Command, exit},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let task = env::args().nth(1);
    let result = match task.as_deref() {
        Some("build")  => build(),
        Some("flash")  => flash(),
        Some("check")  => check(),
        Some("verify") => verify(),
        Some("size")   => size(),
        Some("clean")  => clean(),
        Some("lint")   => lint(),
        _ => {
            eprintln!("Usage: cargo xtask <task>");
            eprintln!();
            eprintln!("Tasks:");
            eprintln!("  build    Build in release mode");
            eprintln!("  flash    Build and flash via elf2uf2-rs");
            eprintln!("  check    Run cargo check");
            eprintln!("  verify   Check .boot2 is at 0x10000000");
            eprintln!("  size     Print binary section sizes");
            eprintln!("  clean    cargo clean");
            eprintln!("  lint     Run fmt + clippy");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        exit(1);
    }
}

// --- tasks -------------------------------------------------------------------

fn build() -> Result<()> {
    run("cargo", &["build", "--release", "-p", "{{project-name}}"])
}

fn flash() -> Result<()> {
    build()?;
    // elf2uf2-rs -d handles waiting for the device and transferring
    run("elf2uf2-rs", &[
        "-d",
        "target/thumbv6m-none-eabi/release/{{project-name}}",
    ])
}

fn check() -> Result<()> {
    run("cargo", &["check", "-p", "{{project-name}}"])
}

fn verify() -> Result<()> {
    build()?;

    let output = Command::new("arm-none-eabi-objdump")
        .args([
            "-h",
            "target/thumbv6m-none-eabi/release/{{project-name}}",
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Find the .boot2 section line and check its VMA address
    let boot2_line = stdout
        .lines()
        .find(|l| l.contains(".boot2"))
        .ok_or("No .boot2 section found in binary")?;

    // objdump -h columns: Idx Name Size VMA LMA File off Algn
    let vma = boot2_line
        .split_whitespace()
        .nth(3)
        .ok_or("Could not parse .boot2 VMA")?;

    if vma == "10000000" {
        println!("✓ .boot2 is at 0x10000000 — OK");
        Ok(())
    } else {
        Err(format!(
            "✗ .boot2 is at 0x{vma}, expected 0x10000000\n\
             Check that memory.x contains the SECTIONS block."
        )
        .into())
    }
}

fn size() -> Result<()> {
    build()?;
    run("arm-none-eabi-size", &[
        "-Ax",
        "target/thumbv6m-none-eabi/release/{{project-name}}",
    ])
}

fn clean() -> Result<()> {
    run("cargo", &["clean"])
}

fn lint() -> Result<()> {
    // fmt — check only, don't rewrite
    run("cargo", &["fmt", "--all", "--", "--check"])?;
    // clippy targeting the embedded target
    run("cargo", &[
        "clippy",
        "-p", "{{project-name}}",
        "--",
        "-D", "warnings",
    ])
}

// --- helpers -----------------------------------------------------------------

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    println!("$ {cmd} {}", args.join(" "));
    let status = Command::new(cmd).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`{cmd}` exited with {status}").into())
    }
}
