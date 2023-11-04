use brrrt_core::{
    elf32::{SectionName, ELF},
    Program, VM,
};
use std::{env, fs};

pub fn load_program() -> Program {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE:");
        eprintln!("\t{}: <PROGRAM_BINFILE>", args[0]);
        panic!("Please, provide program");
    }
    load_program_from(&args[1])
}

pub fn load_program_from(path: &str) -> Program {
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

pub fn load_execution_set(program: &mut Program, vm: &mut VM) -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE:");
        eprintln!("\t{}: <PROGRAM_BINFILE>", args[0]);
        panic!("Please, provide program");
    }
    load_execution_set_(&args[1], program, vm)
}

fn load_execution_set_(
    path: &str,
    program: &mut Program,
    vm: &mut VM,
) -> Result<(), std::io::Error> {
    let executable = std::fs::read(path)?;
    let elf = ELF::parse(&executable).expect("wat");
    if let Some(data) = elf.get(SectionName::Rodata) {
        for (i, &x) in data.get(&executable).iter().enumerate() {
            vm.ram.set_word_at(i as u32, x).expect("unable to set");
        }
    }
    if let Some(data) = elf.get(SectionName::Text) {
        for (i, &x) in data.get(&executable).iter().enumerate() {
            program.write(i as u32, x as u8);
        }
    }
    Ok(())
}
