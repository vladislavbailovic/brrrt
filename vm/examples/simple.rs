use brrrt_vm::{Program, Register, VM};

// https://riscvasm.lucasteske.dev
fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = Program::from_asm(&[
        0x00d00093, // addi x1, x0, 13
        0x00c08113, // addi x2, x1, 12
        0x00282023, // sw x2, 0(x16)
    ]);

    #[cfg(feature = "trace")]
    {
        eprintln!("\n-------------------------------------");
    }

    program.run(&mut vm)?;

    eprintln!("-------------------------------------");
    eprintln!("X01: {} (expected 13)", vm.cpu.register.get(Register::X1));
    eprintln!("X02: {} (expected 25)", vm.cpu.register.get(Register::X2));
    eprintln!("X16: {} (expected 0)", vm.cpu.register.get(Register::X16));
    eprintln!("M@0: {} (expected 25)", vm.ram.byte_at(0).unwrap());

    Ok(())
}
