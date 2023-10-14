#[cfg(feature = "trace")]
use crate::debug;
use crate::{Instruction, Memory, Register, VM};

#[derive(Default)]
pub struct Program {
    end: usize,
    rom: Memory,
}

impl Program {
    pub fn from_asm(asm: &[u32]) -> Self {
        let mut prg: Self = Default::default();
        for (n, x) in asm.iter().enumerate() {
            #[cfg(feature = "trace")]
            {
                eprintln!("{n}: {}", debug::binary(*x, 32));
            }
            prg.rom
                .set_word_at((n * 4) as u32, *x)
                .expect("invalid memory access");
        }
        prg.end = asm.len();
        prg
    }

    pub fn run(&self, vm: &mut VM) -> Result<(), String> {
        for x in 0..100 {
            self.step(vm, x)?;
            if (vm.cpu.register.get(Register::PC) / 4) as usize == self.end {
                break;
            }
        }
        Ok(())
    }

    pub fn step(&self, vm: &mut VM, _iteration: usize) -> Result<(), &str> {
        let pc = vm.cpu.register.get(Register::PC);
        let code = self.rom.word_at(pc).expect("invalid memory access");
        #[cfg(feature = "trace")]
        {
            eprintln!("iteration {} :: PC: {}", _iteration, pc);
        }

        let inst = Instruction::parse(code).expect("should parse");
        #[cfg(feature = "trace")]
        {
            eprintln!("{}: {}", _iteration, debug::binary(code, 32));
            eprintln!("\t{:?}", inst);
        }

        vm.execute(inst)
    }
}
