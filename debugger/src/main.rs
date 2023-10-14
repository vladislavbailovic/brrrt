use brrrt_vm::{Program, Register, VM};

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

    loop {
        program.step(&mut vm, 0)?;
        eprintln!("step!");
        if program.is_done(&vm) {
            break;
        }
    }
    eprintln!("yay");

    Ok(())
}
