use brrrt_core::{debug, risc32i::instruction::Instruction, Register, VM};
use crossterm::{cursor, style, QueueableCommand};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub fn at(pos: Rect, mut what: Vec<String>) {
    let mut stdout = io::stdout();
    for (idx, line) in what.iter_mut().enumerate() {
        line.truncate(pos.w as usize);
        stdout
            .queue(cursor::MoveTo(pos.x, pos.y + idx as u16))
            .expect("unable to move cursor")
            .queue(style::Print(line))
            .expect("unable to print");
    }
    stdout.flush().expect("unable to flush");
}

pub fn memory(vm: &VM) -> Vec<String> {
    let mut out = Vec::new();
    let mut tmp = String::with_capacity(40);
    for pos in 0..24 {
        if pos > 0 && pos % 4 == 0 {
            out.push(tmp.clone());
            tmp = String::new();
        }
        tmp.push_str(&format!(
            "{:02}: {: <18}",
            pos,
            debug::number(
                vm.ram.byte_at(pos).expect("invalid memory access") as u32,
                8
            )
        ));
    }
    out
}

pub fn registers(vm: &VM) -> Vec<String> {
    let mut out = Vec::new();
    let registers = &[Register::X0, Register::X1, Register::X2, Register::X3];
    out.push(format!(
        "PC: {}",
        debug::number(vm.cpu.register.get(Register::PC), 32)
    ));
    for reg in registers {
        out.push(format!(
            "{:?}: {}",
            reg,
            debug::number(vm.cpu.register.get(*reg), 32)
        ));
    }
    out
}

pub fn instruction(instr: &Instruction) -> Vec<String> {
    vec![
        format!("Next: {:?}", instr),
        format!("Raw: {}", debug::number(instr.raw, 32)),
    ]
}

pub fn prompt() -> Vec<String> {
    vec!["> ".to_owned()]
}
