const MAX_NAME_LENGTH: usize = 32;

#[derive(Debug, PartialEq)]
pub enum SectionName {
    Text,
    Rodata,
}

impl TryFrom<String> for SectionName {
    type Error = SectionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            ".text" => Ok(Self::Text),
            ".rodata" => Ok(Self::Rodata),
            _ => Err(SectionError::UnknownSectionName),
        }
    }
}

#[derive(Debug)]
pub struct Section {
    pub name: SectionName,
    header: SectionHeader,
}

impl Section {
    pub fn parse(hdr: SectionHeader, executable: &[u8]) -> Result<Self, SectionError> {
        if executable.is_empty() {
            return Err(SectionError::MissingSectionName);
        }
        let mut name = String::with_capacity(MAX_NAME_LENGTH);
        for x in 0..MAX_NAME_LENGTH {
            if x > executable.len() - 1 {
                return Err(SectionError::InvalidSectionName);
            }
            if let &[c] = &executable[x..x + 1] {
                if c == 0 {
                    break;
                }
                name.push(c as char);
            } else {
                return Err(SectionError::InvalidSectionName);
            }
        }
        if name.len() == 0 {
            return Err(SectionError::MissingSectionName);
        }
        let name: SectionName = name.try_into()?;
        Ok(Self { name, header: hdr })
    }

    pub fn get(&self, executable: &[u8]) -> Vec<u32> {
        let hdr = &self.header;
        let mut data = Vec::with_capacity(hdr.size as usize);
        match &self.name {
            SectionName::Rodata => {
                for x in (hdr.offset..hdr.offset + hdr.size).step_by(hdr.align as usize) {
                    let x = x as usize;
                    if let &[val] = &executable[x..x + 1] {
                        data.push(val as u32)
                    }
                }
            }
            SectionName::Text => {
                for x in hdr.offset..hdr.offset + hdr.size {
                    let x = x as usize;
                    if let &[val] = &executable[x..x + 1] {
                        data.push(val as u32)
                    }
                }
            }
        };
        data
    }
}

#[derive(Default, Debug)]
pub struct SectionHeader {
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

impl SectionHeader {
    pub(crate) fn parse(executable: &[u8]) -> Result<Self, SectionHeaderError> {
        let mut sh: Self = Default::default();
        let mut start = 0;

        sh.typ = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::TypeError))?)
        };
        start += 4;

        sh.flags = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::FlagsError))?)
        };
        start += 4;

        sh.addr = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::AddrError))?)
        };
        start += 4;

        sh.offset = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::OffsetError))?)
        };
        start += 4;

        sh.size = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::SizeError))?)
        };
        start += 4;

        sh.link = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::LinkError))?)
        };
        start += 4;

        sh.info = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::InfoError))?)
        };
        start += 4;

        sh.align = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(entry.try_into().or(Err(SectionHeaderError::AlignError))?)
        };
        start += 4;

        sh.entsize = {
            let entry = &executable[start..start + 4];
            u32::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(SectionHeaderError::EntrySizeError))?,
            )
        };

        Ok(sh)
    }
}

#[derive(Debug)]
pub(crate) enum SectionHeaderError {
    TypeError,
    FlagsError,
    AddrError,
    OffsetError,
    SizeError,
    LinkError,
    InfoError,
    AlignError,
    EntrySizeError,
}

#[derive(Debug)]
pub enum SectionError {
    MissingSectionName,
    InvalidSectionName,
    UnknownSectionName,
}

#[cfg(test)]
mod section {
    use super::*;

    #[test]
    fn parse_empty_should_fail() {
        Section::parse(Default::default(), &Vec::new()).map_or_else(
            |e| match e {
                SectionError::MissingSectionName => assert!(true),
                _ => assert!(false, "unexpected error: {:?}", e),
            },
            |_| assert!(false, "expected failure"),
        );
    }

    #[test]
    fn parse_zero_byte_should_fail() {
        Section::parse(Default::default(), &vec![0]).map_or_else(
            |e| match e {
                SectionError::MissingSectionName => assert!(true),
                _ => assert!(false, "unexpected error: {:?}", e),
            },
            |_| assert!(false, "expected failure"),
        );
    }

    #[test]
    fn parse_garbage_should_fail() {
        Section::parse(Default::default(), String::from("wat").as_bytes()).map_or_else(
            |e| match e {
                SectionError::InvalidSectionName => assert!(true),
                _ => assert!(false, "unexpected error: {:?}", e),
            },
            |_| assert!(false, "expected failure"),
        );
    }

    #[test]
    fn parse_happy_path() {
        Section::parse(
            Default::default(),
            &vec!['.' as u8, 't' as u8, 'e' as u8, 'x' as u8, 't' as u8, 0],
        )
        .map_or_else(
            |e| assert!(false, "expected success: {:?}", e),
            |_| assert!(true, "expected success"),
        );
    }
}
