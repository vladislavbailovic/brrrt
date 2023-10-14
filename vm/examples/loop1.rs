use brrrt_vm::{Program, Register, VM};

/*
    // https://riscvasm.lucasteske.dev
    addi x1, x0, 12
    loop: addi x2, x2, 1
    bne x1, x2, loop
    addi x1, x1, 1
    add x2, x1, x2
    sw x2, 0(x16)
*/
fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = Program::from_asm(&[
        0x00c00093, 0x00110113, 0xfe209ee3, 0x00108093, 0x00208133, 0x00282023,
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
