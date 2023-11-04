use brrrt_core::{
    elf32::{Error, SectionName, ELF},
    Program, VM,
};
use std::{env, fs};

pub enum RuntimeError {
    UsageError,
    ReadError,
    LoadError,
}

impl From<std::io::Error> for RuntimeError {
    fn from(_e: std::io::Error) -> Self {
        #[cfg(feature = "trace")]
        {
            eprintln!("Error loading file: {:?}", _e);
        }
        Self::ReadError
    }
}

impl From<Error> for RuntimeError {
    fn from(_e: Error) -> Self {
        #[cfg(feature = "trace")]
        {
            eprintln!("Error loading file: {:?}", _e);
        }
        Self::LoadError
    }
}

impl From<&str> for RuntimeError {
    fn from(_e: &str) -> Self {
        #[cfg(feature = "trace")]
        {
            eprintln!("Error loading file: {:?}", _e);
        }
        Self::LoadError
    }
}

impl From<RuntimeError> for String {
    fn from(e: RuntimeError) -> Self {
        format!(
            "Runtime error: {}",
            match e {
                RuntimeError::UsageError => "unexpected usage",
                RuntimeError::ReadError => "read error",
                RuntimeError::LoadError => "load error",
            }
        )
    }
}

pub fn load_program() -> Result<Program, RuntimeError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE:");
        eprintln!("\t{}: <PROGRAM_BINFILE>", args[0]);
        Err(RuntimeError::UsageError)
    } else {
        load_program_from(&args[1])
    }
}

fn load_program_from(path: &str) -> Result<Program, RuntimeError> {
    let mut prg: Program = Default::default();
    let src = fs::read(path)?.into_iter().enumerate();
    for (i, x) in src {
        prg.write(i as u32, x);
    }
    Ok(prg)
}

pub fn load_execution_set(program: &mut Program, vm: &mut VM) -> Result<(), RuntimeError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE:");
        eprintln!("\t{}: <PROGRAM_BINFILE>", args[0]);
        Err(RuntimeError::UsageError)
    } else {
        load_execution_set_from(&args[1], program, vm)
    }
}

fn load_execution_set_from(
    path: &str,
    program: &mut Program,
    vm: &mut VM,
) -> Result<(), RuntimeError> {
    let executable = std::fs::read(path)?;
    let elf = ELF::parse(&executable)?;
    if let Some(data) = elf.get(SectionName::Rodata) {
        for (i, &x) in data.get(&executable).iter().enumerate() {
            vm.ram.set_word_at(i as u32, x)?;
        }
    }
    if let Some(data) = elf.get(SectionName::Text) {
        for (i, &x) in data.get(&executable).iter().enumerate() {
            program.write(i as u32, x as u8);
        }
    }
    Ok(())
}
