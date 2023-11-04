use brrrt_core::{Program, VM};
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
    let mut e: ELFHeader = Default::default();

    if !has_valid_header(&executable) {
        panic!("Aborting");
    }
    e.entry = {
        let entry = &executable[0x18..0x18 + 4];
        u32::from_le_bytes(entry.try_into().expect("program entry point"))
    };

    e.phoff = {
        let entry = &executable[0x1C..0x1C + 4];
        u32::from_le_bytes(entry.try_into().expect("section header table"))
    };
    e.phentsize = {
        let entry = &executable[0x2A..0x2A + 2];
        u16::from_le_bytes(entry.try_into().expect("section header size"))
    };
    e.phnum = {
        let entry = &executable[0x2C..0x2C + 2];
        u16::from_le_bytes(entry.try_into().expect("section number of entities"))
    };

    e.shoff = {
        let entry = &executable[0x20..0x20 + 4];
        u32::from_le_bytes(entry.try_into().expect("section header table"))
    };
    e.shentsize = {
        let entry = &executable[0x2E..0x2E + 2];
        u16::from_le_bytes(entry.try_into().expect("section header size"))
    };
    e.shnum = {
        let entry = &executable[0x30..0x30 + 2];
        u16::from_le_bytes(entry.try_into().expect("section number of entities"))
    };
    e.shstrndx = {
        let entry = &executable[0x32..0x32 + 2];
        u16::from_le_bytes(entry.try_into().expect("section number of entities"))
    };

    eprintln!("ELF Header: {:?}", e);

    // TODO: Program header

    let mut section_headers = Vec::with_capacity(e.shnum as usize);
    if e.shnum > 0 {
        let shoffstart = {
            let start = (e.shstrndx * e.shentsize + e.shoff as u16) as usize + 4 * 4; // "offset" is fifth 4-byte field
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().expect("string header offset"))
        };
        for h in 1..e.shnum {
            let init = ((h * e.shentsize) + e.shoff as u16) as usize;
            let mut start = init;
            let mut sh: SectionHeader = Default::default();

            let mut nstart = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header name"))
            } as usize;
            nstart += shoffstart as usize;
            sh.name = {
                let mut name = String::new();
                loop {
                    if let &[x] = &executable[nstart..nstart + 1] {
                        if x == 0 {
                            break;
                        }
                        name.push(x as char);
                    } else {
                        break;
                    }
                    nstart += 1;
                }
                name
            };
            start += 4;

            sh.typ = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header type"))
            };
            start += 4;

            sh.flags = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header flags"))
            };
            start += 4;

            sh.addr = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header addr"))
            };
            start += 4;

            sh.offset = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header offset"))
            };
            start += 4;

            sh.size = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header size"))
            };
            start += 4;

            sh.link = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header link"))
            };
            start += 4;

            sh.info = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header info"))
            };
            start += 4;

            sh.align = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header align"))
            };
            start += 4;

            sh.entsize = {
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().expect("section header entsize"))
            };
            // start += 4;

            eprintln!("\t- Section {:?}", sh);
            section_headers.push(sh);
        }
    } else {
        panic!("No section headers!");
    }

    for s in section_headers {
        if ".rodata" == s.name {
            eprintln!("Initializing RODATA section");
            for x in (s.offset..s.offset + s.size).step_by(s.align as usize) {
                let x = x as usize;
                if let &[val] = &executable[x..x + 1] {
                    vm.ram
                        .set_word_at(x as u32 - s.offset, val as u32)
                        .expect("error setting memory");
                } else {
                    panic!("Could not load data value")
                };
            }
        }
        if ".text" == s.name {
            for x in s.offset..s.offset + s.size {
                let x = x as usize;
                if let &[val] = &executable[x..x + 1] {
                    program.write(x as u32 - s.offset, val);
                } else {
                    panic!("Could not load program")
                };
            }
        }
    }
    Ok(())
}

#[derive(Default, Debug)]
struct ELFHeader {
    entry: u32,
    phoff: u32,
    phentsize: u16,
    phnum: u16,
    shoff: u32,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

#[derive(Default, Debug)]
struct SectionHeader {
    name: String,
    typ: u32,
    flags: u32,
    addr: u32,
    offset: u32,
    size: u32,
    link: u32,
    info: u32,
    align: u32,
    entsize: u32,
}

fn has_valid_header(executable: &[u8]) -> bool {
    // magic
    if let &[0x7F, 0x45, 0x4C, 0x46] = &executable[0..4] {
        // 32-bit
        if let &[1] = &executable[4..5] {
            // risc
            if let &[0xF3] = &executable[0x12..0x13] {
                return true;
            } else {
                eprintln!("Not RISC-V");
            }
        } else {
            eprintln!("Not 32-bit");
        }
    } else {
        eprintln!("Not ELF");
    }
    false
}
