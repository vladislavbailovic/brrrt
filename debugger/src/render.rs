use brrrt_core::{debug, risc32i::instruction::Instruction, Register, VM};

pub fn memory(vm: &VM) {
    eprintln!();
    for pos in 0..24 {
        if pos > 0 && pos % 4 == 0 {
            eprintln!();
        }
        eprint!(
            "{:02}: {: <18}",
            pos,
            debug::number(
                vm.ram.byte_at(pos).expect("invalid memory access") as u32,
                8
            )
        );
    }
    eprintln!();
}

pub fn registers(vm: &VM) {
    let registers = &[Register::X0, Register::X1, Register::X2, Register::X3];
    eprintln!(
        "PC: {}",
        debug::number(vm.cpu.register.get(Register::PC), 32)
    );
    for reg in registers {
        eprintln!(
            "{:?}: {}",
            reg,
            debug::number(vm.cpu.register.get(*reg), 32)
        );
    }
}

pub fn instruction(instr: &Instruction) {
    eprintln!("Next: {:?}", instr);
    eprintln!("Raw: {}", debug::number(instr.raw, 32));
}
