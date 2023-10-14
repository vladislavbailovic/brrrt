use brrrt::{
    risc32i::{instr::builder::Builder, instr::operation::*, instr::part::Part},
    Program, Register, VM,
};

fn from_builder() -> Vec<u32> {
    vec![
        // X1 = 13
        Builder::opcode(Operation::LUI)
            .pack(Part::Dest, Register::X1 as u32)
            .pack(Part::Imm3112, 13)
            .build(),
        // X2 = X1 + 12
        Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Funct3, 0b000)
            .pack(Part::Imm110, 12)
            .pack(Part::Reg1, Register::X1 as u32)
            .pack(Part::Dest, Register::X2 as u32)
            .build(),
        // X2 => m@[X16]
        Builder::opcode(Operation::Store)
            .pack(Part::Imm110, 0)
            .pack(Part::Reg1, Register::X16 as u32)
            .pack(Part::Reg2, Register::X2 as u32)
            .build(),
    ]
}

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = Program::from_asm(&from_builder());

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
