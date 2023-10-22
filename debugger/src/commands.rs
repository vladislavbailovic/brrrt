use brrrt_core::Register;

#[derive(Debug)]
pub enum Command {
    SetRegister(Register, u32),
    SetMemory(u32, u8),
}

pub fn parse_command(input: &str) -> Option<Command> {
    let mut t = Tokenizer::new(input);
    if let Some(Token::Bang) = t.next() {
        match t.next() {
            Some(Token::Plus) => {
                let register: Option<Register> = match t.next() {
                    Some(Token::Identifier(x)) => x.try_into().ok(),
                    Some(Token::Number(n)) => n.try_into().ok(),
                    _ => None,
                };
                let value = match t.next() {
                    Some(Token::Number(n)) => Some(n),
                    _ => None,
                }?;
                return Some(Command::SetRegister(register?, value));
            }
            Some(Token::At) => {
                let address = match t.next() {
                    Some(Token::Number(n)) => Some(n),
                    None => None,
                    _ => {
                        return None;
                    }
                }?;
                let byte = match t.next() {
                    Some(Token::Number(n)) => Some(n as u8),
                    None => None,
                    _ => {
                        return None;
                    }
                }?;
                return Some(Command::SetMemory(address, byte));
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

#[cfg(test)]
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
    fn parser_returns_none_on_invalid_memory_command() {
        let cmd = parse_command("!@ wat");
        assert!(cmd.is_none());
    }
}
