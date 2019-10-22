extern crate cc;

use std::fs::File;
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-env-changed=LOG");
    println!("cargo:rerun-if-env-changed=BOARD");
    println!("cargo:rerun-if-env-changed=FEATURES");

    let arch: String = std::env::var("ARCH").unwrap();
    let _board: String = std::env::var("BOARD").unwrap();

    if cfg!(feature = "link_user") {
        println!("cargo:rerun-if-env-changed=USER_IMG");
        if let Ok(user_img) = std::env::var("USER_IMG") {
            println!("cargo:rerun-if-changed={}", user_img);
        }
    }

    match arch.as_str() {
        "x86_64" => {
            gen_vector_asm().unwrap();
        }
        "riscv32" => {}
        "riscv64" => {}
        "mipsel" => {}
        "aarch64" => {
            println!("cargo:rerun-if-env-changed=SMP");
        }
        _ => panic!("Unknown arch {}", arch),
    }
}

/// Generate assembly file for x86_64 trap vector
fn gen_vector_asm() -> Result<()> {
    let mut f = File::create("src/arch/x86_64/interrupt/vector.asm").unwrap();

    writeln!(f, "# generated by build.rs - do not edit")?;
    writeln!(f, ".section .text")?;
    writeln!(f, ".intel_syntax noprefix")?;
    for i in 0..256 {
        writeln!(f, "vector{}:", i)?;
        if !(i == 8 || (i >= 10 && i <= 14) || i == 17) {
            writeln!(f, "\tpush 0")?;
        }
        writeln!(f, "\tpush {}", i)?;
        writeln!(f, "\tjmp __alltraps")?;
    }

    writeln!(f, "\n.section .rodata")?;
    writeln!(f, ".global __vectors")?;
    writeln!(f, "__vectors:")?;
    for i in 0..256 {
        writeln!(f, "\t.quad vector{}", i)?;
    }
    Ok(())
}
