use brrrt_core::{Program, VM};
use std::{fs, io};

use crossterm::{execute, terminal};

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
            execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))
                .expect("unable to clear");
            let prompt_top = if !program.is_done(&vm) {
                let instr = program.peek(&vm)?;
                render::at(
                    render::Rect {
                        x: 0,
                        y: 6,
                        w: 80,
                        h: 2,
                    },
                    render::instruction(&instr),
                );
                8
            } else {
                6
            };

            let pos = render::Rect {
                x: 0,
                y: 0,
                w: 86,
                h: 4,
            };
            render::at(pos, render::memory(&vm));
            let pos = render::Rect {
                x: 94,
                y: 0,
                w: 50,
                h: 4,
            };
            render::at(pos, render::registers(&vm));

            render::at(
                render::Rect {
                    x: 0,
                    y: prompt_top,
                    w: 80,
                    h: 1,
                },
                render::prompt(),
            );
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("ERROR: unable to read input");
                continue;
            }
            let action = match input.chars().next() {
                Some('!') => match apply_command(&input, &mut vm) {
                    None => Action::Input,
                    Some(action) => action,
                },
                Some('q') => Action::Quit,
                Some('\n') => Action::Step,
                _ => {
                    eprintln!("WARNING: unknown command");
                    Action::Input
                }
            };
            match action {
                Action::Quit => {
                    quit = true;
                    break;
                }
                Action::Step => break,
                Action::Input => continue,
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
    Input,
    Step,
    Quit,
}

fn apply_command(input: &str, vm: &mut VM) -> Option<Action> {
    let cmd = parse_command(input)?;
    match cmd {
        Command::SetRegister(reg, val) => {
            vm.cpu.register.set(reg, val);
        }
        Command::SetMemory(address, byte) => {
            vm.ram
                .set_byte_at(address, byte)
                .expect("invalid memory access");
        }
    }
    None
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
