use brrrt::{
    risc32i::{
        instr::instruction::Instruction, instr::operation::*,
    },
    Cpu, Register,
};

// tests
#[cfg(test)]
mod branches;
#[cfg(test)]
mod immediate;
#[cfg(test)]
mod immediate_math;
#[cfg(test)]
mod jumps;
#[cfg(test)]
mod load;
#[cfg(test)]
mod math;
#[cfg(test)]
mod store;
#[cfg(test)]
mod memory;

fn main() {
    println!("yo");
}
