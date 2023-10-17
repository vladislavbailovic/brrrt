use brrrt_core::{Program, Register, VM};

/*
    // https://riscvasm.lucasteske.dev
    addi x1, x0, 13
    addi x2, x0, 12
    j end
    addi x2, x0, 161
    end: sw x2, 0(x16)
*/
fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = Program::from_asm(&[0x00d00093, 0x00c00113, 0x0080006f, 0x0a100113, 0x00282023]);

    #[cfg(feature = "trace")]
    {
        eprintln!("\n-------------------------------------");
    }

    program.run(&mut vm)?;

    eprintln!("-------------------------------------");
    eprintln!("X01: {} (expected 13)", vm.cpu.register.get(Register::X1));
    eprintln!("X02: {} (expected 12)", vm.cpu.register.get(Register::X2));
    eprintln!("X16: {} (expected 0)", vm.cpu.register.get(Register::X16));
    eprintln!("M@0: {} (expected 12)", vm.ram.byte_at(0).unwrap());

    Ok(())
}
