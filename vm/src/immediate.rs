#[cfg(test)]
mod lui {
    use crate::risc32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

    #[test]
    fn load_simple() {
        let i = Instruction::parse(
            Builder::opcode(Operation::LUI)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Imm3112, 1312)
                .build(),
        )
        .expect("should parse");

        let mut vm: VM = Default::default();

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 1312);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
    }
}

#[cfg(test)]
mod auipc {
    use crate::risc32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

    #[test]
    fn load_pc0() {
        let i = Instruction::parse(
            Builder::opcode(Operation::AUIPC)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Imm3112, 1312)
                .build(),
        )
        .expect("should parse");

        let mut vm: VM = Default::default();

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 1312);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
    }

    #[test]
    fn load_pc_nonzero() {
        let i = Instruction::parse(
            Builder::opcode(Operation::AUIPC)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Imm3112, 13)
                .build(),
        )
        .expect("should parse");

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::PC, 12);

        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 25);
        assert_eq!(vm.cpu.register.get(Register::PC), 16);
    }
}
