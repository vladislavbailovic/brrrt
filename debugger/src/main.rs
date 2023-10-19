use brrrt_core::{Program, VM};
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
            let mut out = Vec::new();
            if !program.is_done(&vm) {
                let instr = program.peek(&vm)?;
                out.extend(render::instruction(&instr));
            }

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("ERROR: unable to read input");
                continue;
            }
            let action = match input.chars().next() {
                Some('!') => match apply_command(&input, &mut vm) {
                    None => Action::Input,
                    Some(Action::Render(view)) => {
                        out.extend(match view {
                            View::Memory => render::memory(&vm),
                            View::Registers => render::registers(&vm),
                        });
                        Action::Input
                    }
                    Some(action) => action,
                },
                Some('q') => Action::Quit,
                Some('\n') => Action::Input,
                _ => {
                    eprintln!("WARNING: unknown command");
                    Action::Input
                }
            };
            eprintln!("{}", out.join("\n"));
            match action {
                Action::Quit => {
                    quit = true;
                    break;
                }
                Action::Input => continue,
                Action::Render(_) => unreachable!(),
            };
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

enum Action {
    Render(View),
    Input,
    Quit,
}

enum View {
    Memory,
    Registers,
}

fn apply_command(input: &str, vm: &mut VM) -> Option<Action> {
    let cmd = parse_command(input);
    if cmd.is_none() {
        return None;
    }
    match cmd.unwrap() {
        Command::SetRegister(reg, val) => {
            vm.cpu.register.set(reg, val);
            None
        }
        Command::SetMemory(address, byte) => {
            vm.ram
                .set_byte_at(address, byte)
                .expect("invalid memory access");
            Some(Action::Render(View::Memory))
        }
        Command::ShowMemory => Some(Action::Render(View::Memory)),
        Command::ShowRegisters => Some(Action::Render(View::Registers)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use brrrt_core::Register;

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
