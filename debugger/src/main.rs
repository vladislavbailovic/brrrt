use brrrt_cli::load_program;
use brrrt_core::VM;
use std::io;

use crossterm::{execute, terminal};

mod commands;
use commands::{parse_command, Command};

mod render;

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let mut debug_vm: VM = Default::default();
    let program = load_program();

    vm.cpu.initialize();
    debug_vm.cpu.initialize();

    let mut quit = false;
    let mut outcome = Vec::new();
    while !quit {
        if !program.is_done(&debug_vm) {
            program.step(&mut debug_vm, 0)?;
        }
        loop {
            execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))
                .expect("unable to clear");
            let prompt_top = if !program.is_done(&vm) {
                let instr = debug_vm.last();
                let pos = if let Some(instr) = instr {
                    render::at(render::Position { x: 0, y: 6 }, render::instruction(&instr));
                    8
                } else {
                    6
                };
                let debug = debug_vm.debug();
                let line_count = debug.len();
                if !debug.is_empty() {
                    render::at(render::Position { x: 0, y: pos }, debug);
                }
                pos + line_count as u16
            } else {
                6
            };

            let pos = render::Position { x: 0, y: 0 };
            render::at(pos, render::memory(&vm));
            let pos = render::Position { x: 100, y: 0 };
            render::at(pos, render::registers(&vm));

            if !outcome.is_empty() {
                render::at(
                    render::Position {
                        x: 0,
                        y: prompt_top + 1,
                    },
                    outcome.clone(),
                );
                outcome.clear();
            }

            render::at(
                render::Position {
                    x: 0,
                    y: prompt_top,
                },
                render::prompt(),
            );
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                outcome.extend_from_slice(&render::error("unable to read input"));
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
                    outcome.extend_from_slice(&render::warning("unknown command"));
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
                Action::Inspect(val) => {
                    outcome.extend_from_slice(&val);
                    continue;
                }
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
    Inspect(Vec<String>),
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
        Command::DumpRegister(reg) => {
            return Some(Action::Inspect(render::register(reg, vm)));
        }
        Command::DumpMemory(start_addr) => {
            return Some(Action::Inspect(render::memory_at(start_addr, vm)));
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
