use brrrt_core::{Program, VM};
use std::fs;

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
    let program = load_program("data/c/simple.bin");

    program.run(&mut vm)?;
    Ok(())
}
