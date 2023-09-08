mod risc32i;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Uint(u64),
    Sint(i64),
    Float(f64),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Push(Operand),
    Plus,
}

const PROGRAM_SIZE: usize = 256;
const STACK_SIZE: usize = 256;

#[derive(Debug)]
struct Machine {
    program: Vec<Instruction>,
    ip: usize,
    stack: Vec<Operand>,
}

impl Machine {
    fn new() -> Self {
        Self {
            program: Vec::with_capacity(PROGRAM_SIZE),
            ip: 0,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    fn add_instr(&mut self, instr: Instruction) {
        self.program.push(instr);
    }

    fn load_program(&mut self, program: &mut [Instruction]) {
        for p in program {
            self.add_instr(*p);
        }
    }

    fn run(&mut self) -> Result<(), String> {
        for i in self.ip..self.program.len() {
            match self.program[i] {
                Instruction::Push(val) => {
                    if self.stack.len() >= STACK_SIZE {
                        return Err("stack overflow".into());
                    }
                    self.stack.push(val);
                    self.ip += 1;
                }
                Instruction::Plus => {
                    if self.stack.len() < 2 {
                        return Err("stack underflow".into())
                    }
                    let rh = self.stack.pop().unwrap();
                    let lh = self.stack.pop().unwrap();
                    match (lh, rh) {
                        (Operand::Uint(left), Operand::Uint(right)) => self.stack.push(Operand::Uint(left+right)),
                        (Operand::Sint(left), Operand::Sint(right)) => self.stack.push(Operand::Sint(left+right)),
                        (Operand::Float(left), Operand::Float(right)) => self.stack.push(Operand::Float(left+right)),
                        (left, right) => {
                            return Err(format!("mismatched types: {left:?}, {right:?}"));
                        }
                    }
                    self.ip += 1;
                }
                _ => {
                    self.ip += 1;
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut m = Machine::new();
    m.load_program(&mut [
        Instruction::Push(Operand::Sint(13)),
        Instruction::Push(Operand::Sint(12)),
        Instruction::Plus,
    ]);
    m.run()?;
    println!("{:?}", m);
    Ok(())
}
