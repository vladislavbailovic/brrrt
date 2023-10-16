use brrrt_vm::{Program, VM};
use std::{fs, io};

mod commands;
use commands::{parse_command, Command};

mod render;

fn load_program(path: &str) -> Program {
    let mut prg: Program = Default::default();
    let src = fs::read(path)
        .expect("Unable to read file")
        .into_iter()
        .enumerate();
    for (i, x) in src {
        prg.write(i as u32, x);
    }
    prg
}

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = load_program("asm/simple.bin");

    let mut quit = false;
    while !quit {
        loop {
            render::registers(&vm);
            if !program.is_done(&vm) {
                let instr = program.peek(&vm)?;
                render::instruction(&instr);
            }

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("ERROR: unable to read input");
                continue;
            }
            match input.chars().next() {
                Some('!') => {
                    apply_command(&input, &mut vm);
                    continue;
                }
                Some('q') => {
                    quit = true;
                    break;
                }
                Some('\n') => {
                    break;
                }
                _ => {
                    eprintln!("WARNING: unknown command");
                    continue;
                }
            }
        }
        if quit {
            break;
        }

        if !program.is_done(&vm) {
            program.step(&mut vm, 0)?;
        }
    }

    Ok(())
}

fn apply_command(input: &str, vm: &mut VM) {
    let cmd = parse_command(input);
    if cmd.is_none() {
        return;
    }
    match cmd.unwrap() {
        Command::SetRegister(reg, val) => {
            vm.cpu.register.set(reg, val);
        }
        Command::SetMemory(address, byte) => {
            vm.ram
                .set_byte_at(address, byte)
                .expect("invalid memory access");
            render::memory(vm);
        }
        Command::ShowMemory => {
            render::memory(vm);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use brrrt_vm::{Register};

    #[test]
    fn apply_set_register_command() {
        let mut vm: VM = Default::default();
        assert_eq!(0, vm.cpu.register.get(Register::PC));

        apply_command("!+ PC 161", &mut vm);
        assert_eq!(161, vm.cpu.register.get(Register::PC));
    }

    #[test]
    fn apply_set_memory_command() {
        let mut vm: VM = Default::default();
        assert_eq!(0, vm.ram.byte_at(161).unwrap());

        apply_command("!@ 161 13", &mut vm);
        assert_eq!(13, vm.ram.byte_at(161).unwrap());
    }
}
