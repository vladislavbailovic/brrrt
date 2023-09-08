mod risc32i;
use risc32i::{
    *, instr::part::Part, instr::format::Format, instr::builder::Builder};

fn main() -> Result<(), String>  {
    let i = Instruction::parse(
        Builder::new(0b1100011).build()
    )?;
    let num = i.get(Part::Opcode).unwrap();
    let value = i.value(Part::Opcode).unwrap();
    eprintln!("{:?}, value: {}, num: {}", i, value, num);
    eprintln!("{:?}", Part::Null);
    eprintln!("{:?}", Format::Jump);
    Ok(())
}
