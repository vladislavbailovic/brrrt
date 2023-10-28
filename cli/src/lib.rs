use brrrt_core::Program;
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
