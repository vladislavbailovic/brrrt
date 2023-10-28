use brrrt_core::{debug, rv32i::instruction::Instruction, Register, VM};
use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub fn at(pos: Position, mut what: Vec<String>) {
    let mut stdout = io::stdout();

    for (idx, line) in what.iter_mut().enumerate() {
        stdout
            .queue(cursor::MoveTo(pos.x, pos.y + idx as u16))
            .expect("unable to move cursor")
            .queue(style::Print(line))
            .expect("unable to print");
    }
    stdout.flush().expect("unable to flush");
}

pub fn memory(vm: &VM) -> Vec<String> {
    memory_at(0, vm)
}

pub fn memory_at(start_addr: u32, vm: &VM) -> Vec<String> {
    let mut out = Vec::new();
    let mut tmp = String::with_capacity(40);
    for pos in start_addr..start_addr + 24 {
        if pos > 0 && pos % 4 == 0 {
            out.push(tmp.clone());
            tmp = String::new();
        }
        tmp.push_str(&format!(
            "{} {: <18}",
            format!("{:04}:", pos).dark_grey(),
            debug::number(
                vm.ram.byte_at(pos).expect("invalid memory access") as u32,
                8
            )
        ));
    }
    out
}

pub fn register(reg: Register, vm: &VM) -> Vec<String> {
    vec![format!(
        "{} {}",
        format!("{:?}:", reg).dark_green(),
        debug::number(vm.cpu.register.get(reg), 32)
    )]
}

pub fn registers(vm: &VM) -> Vec<String> {
    let mut out = Vec::new();
    let registers = &[Register::X0, Register::X1, Register::X2, Register::X3];
    out.push(format!(
        "{} {}",
        "PC:".dark_yellow(),
        debug::number(vm.cpu.register.get(Register::PC), 32)
    ));
    for reg in registers {
        out.push(format!(
            "{} {}",
            format!("{:?}:", reg).white(),
            debug::number(vm.cpu.register.get(*reg), 32)
        ));
    }
    out
}

pub fn instruction(instr: &Instruction) -> Vec<String> {
    vec![
        format!("{} {:?}", "next:".dark_yellow(), instr),
        format!("{}  {}", "raw:".white(), debug::number(instr.raw, 32)),
    ]
}

pub fn prompt() -> Vec<String> {
    vec!["> ".to_owned()]
}
