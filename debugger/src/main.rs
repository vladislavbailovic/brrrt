use brrrt_vm::{debug, Program, Register, VM};

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

    let registers = &[Register::X0, Register::X1, Register::X2, Register::X3];

    loop {
        let instr = program.peek(&vm)?;
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
        eprintln!("Next: {:?}", instr);
        eprintln!("Raw: {}", debug::binary(instr.raw, 32));
        program.step(&mut vm, 0)?;
        eprintln!("step!");
        if program.is_done(&vm) {
            break;
        }
    }

    let mut pos = 0;
    for i in 0..6 {
        for j in 0..4 {
            eprint!(
                "{:02}: {: <18}",
                pos,
                debug::number(vm.ram.byte_at(pos)? as u32, 8)
            );
            pos += 1;
        }
        eprintln!();
    }

    Ok(())
}
