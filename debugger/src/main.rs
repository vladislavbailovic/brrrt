use brrrt_vm::{debug, Program, Register, VM};
use std::{fs, io};

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
    let program = load_program("asm/loop2.bin");

    let registers = &[Register::X0, Register::X1, Register::X2, Register::X3];

    let mut quit = false;
    while !quit {
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
            eprintln!("Raw: {}", debug::number(instr.raw, 32));

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

fn show_memory(vm: &VM) {
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
            show_memory(vm);
        }
        Command::ShowMemory => {
            show_memory(vm);
        }
    }
}

#[derive(Debug)]
enum Command {
    SetRegister(Register, u32),
    SetMemory(u32, u8),
    ShowMemory,
}

fn parse_command(input: &str) -> Option<Command> {
    let mut t = Tokenizer::new(input);
    if let Some(Token::Bang) = t.next() {
        match t.next() {
            Some(Token::Plus) => {
                let register: Option<Register> = match t.next() {
                    Some(Token::Identifier(regname)) => {
                        if "PC" == &regname {
                            Some(Register::PC)
                        } else {
                            let regname = regname.to_lowercase();
                            let first = regname.chars().next();
                            if Some('x') == first && regname.len() > 1 {
                                regname
                                    .strip_prefix("x")
                                    .expect("invalid register")
                                    .parse::<u32>()
                                    .expect("invalid register")
                                    .try_into()
                                    .ok()
                            } else {
                                None
                            }
                        }
                    }
                    Some(Token::Number(n)) => n.try_into().ok(),
                    _ => None,
                };
                if register.is_none() {
                    return None;
                }
                let value = match t.next() {
                    Some(Token::Number(n)) => Some(n),
                    _ => None,
                };
                if value.is_none() {
                    return None;
                }
                return Some(Command::SetRegister(register.unwrap(), value.unwrap()));
            }
            Some(Token::At) => {
                let address = match t.next() {
                    Some(Token::Number(n)) => Some(n),
                    None => None,
                    _ => {
                        return None;
                    }
                };
                let byte = match t.next() {
                    Some(Token::Number(n)) => Some(n as u8),
                    None => None,
                    _ => {
                        return None;
                    }
                };
                if address.is_none() && byte.is_none() {
                    return Some(Command::ShowMemory);
                }
                return Some(Command::SetMemory(address.unwrap(), byte.unwrap()));
            }
            _ => return None,
        }
    }
    None
}

use std::str::Chars;
#[derive(Debug, PartialEq)]
enum Token {
    Bang,
    Plus,
    At,
    Number(u32),
    Identifier(String),
}

struct Tokenizer<'a> {
    chars: Chars<'a>,
    skip: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.trim_start().chars(),
            skip: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        for _ in 0..self.skip {
            self.chars.next();
        }
        self.skip = 0;
        match self.chars.next() {
            None => None,
            Some('!') => Some(Token::Bang),
            Some('+') => Some(Token::Plus),
            Some('@') => Some(Token::At),
            Some(c) if c.is_numeric() => {
                let mut rest = vec![c];
                rest.extend(
                    self.chars
                        .clone()
                        .take_while(|x| x.is_numeric())
                        .collect::<Vec<char>>(),
                );
                self.skip = rest.len();
                let rest: String = rest.into_iter().collect();
                Some(Token::Number(rest.parse::<u32>().expect("invalid number")))
            }
            Some(c) if c.is_alphabetic() => {
                let mut rest = vec![c];
                rest.extend(
                    self.chars
                        .clone()
                        .take_while(|x| !x.is_whitespace())
                        .collect::<Vec<char>>(),
                );
                self.skip = rest.len();
                let rest: String = rest.into_iter().collect();
                Some(Token::Identifier(rest))
            }
            _ => self.next(),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn tokenizer_straightforward() {
        let mut t = Tokenizer::new("!+ 12 whatever @");

        assert_eq!(Some(Token::Bang), t.next());
        assert_eq!(Some(Token::Plus), t.next());
        assert_eq!(Some(Token::Number(12)), t.next());
        assert_eq!(Some(Token::Identifier("whatever".to_string())), t.next());
    }

    #[test]
    fn parser_returns_none_on_bad_command() {
        assert!(parse_command("wat").is_none());
    }

    #[test]
    fn parser_parses_register_number_command() {
        let cmd = parse_command("!+ 1 12");

        assert!(cmd.is_some());
        if let Command::SetRegister(reg, val) = cmd.unwrap() {
            assert_eq!(reg, Register::X1);
            assert_eq!(val, 12);
        } else {
            assert!(false, "unknown command");
        }
    }

    #[test]
    fn parser_parses_register_by_name_command_pc() {
        let cmd = parse_command("!+ PC 13");

        assert!(cmd.is_some());
        if let Command::SetRegister(reg, val) = cmd.unwrap() {
            assert_eq!(reg, Register::PC);
            assert_eq!(val, 13);
        } else {
            assert!(false, "unknown command");
        }
    }

    #[test]
    fn parser_parses_register_by_name_command_x() {
        let cmd = parse_command("!+ X12 13");

        assert!(cmd.is_some());
        if let Command::SetRegister(reg, val) = cmd.unwrap() {
            assert_eq!(reg, Register::X12);
            assert_eq!(val, 13);
        } else {
            assert!(false, "unknown command");
        }
    }

    #[test]
    fn parser_parses_memory_set_command() {
        let cmd = parse_command("! @ 1312 161");

        assert!(cmd.is_some());
        if let Command::SetMemory(address, byte) = cmd.unwrap() {
            assert_eq!(address, 1312);
            assert_eq!(byte, 161);
        } else {
            assert!(false, "unknown command");
        }
    }

    #[test]
    fn parser_parses_memory_show_command() {
        let cmd = parse_command("!@");

        assert!(cmd.is_some());
        if let Command::ShowMemory = cmd.unwrap() {
            assert!(true);
        } else {
            assert!(false, "unknown command");
        }
    }

    #[test]
    fn parser_returns_none_on_invalid_memory_command() {
        let cmd = parse_command("!@ wat");
        assert!(cmd.is_none());
    }

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
