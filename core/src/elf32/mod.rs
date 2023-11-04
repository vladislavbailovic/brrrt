mod header;
mod section;

use header::{ELFHeader, ELFHeaderError};
pub use section::SectionName;
use section::{Section, SectionHeader, SectionHeaderError, SectionNameError};

#[derive(Debug)]
pub enum Error {
    HeaderParseError,
    SectionParseError,
}

impl From<ELFHeaderError> for Error {
    fn from(_e: ELFHeaderError) -> Self {
        #[cfg(feature = "trace")]
        {
            eprintln!("ELF Header error: {:?}", _e);
        }
        Self::HeaderParseError
    }
}

impl From<SectionHeaderError> for Error {
    fn from(_e: SectionHeaderError) -> Self {
        #[cfg(feature = "trace")]
        {
            eprintln!("Section Header error: {:?}", _e);
        }
        Self::SectionParseError
    }
}

impl From<SectionNameError> for Error {
    fn from(_e: SectionNameError) -> Self {
        #[cfg(feature = "trace")]
        {
            eprintln!("Section name error: {:?}", _e);
        }
        Self::SectionParseError
    }
}

#[derive(Debug, Default)]
pub struct ELF {
    header: ELFHeader,
    sections: Vec<Section>,
}

impl ELF {
    pub fn get(&self, s: SectionName) -> Option<&Section> {
        self.sections.iter().find(|&x| x.name == s)
    }

    pub fn parse(executable: &[u8]) -> Result<Self, Error> {
        ELFHeader::is_valid(executable)?;
        let mut e: ELF = Self {
            header: ELFHeader::parse(executable)?,
            ..Default::default()
        };

        if e.header.shnum > 0 {
            let names_offset = {
                let field_off = 4 * 4; // "offset" is fifth 4-byte field
                let start = (e.header.shstrndx * e.header.shentsize + e.header.shoff as u16)
                    as usize
                    + field_off;
                let entry = &executable[start..start + 4];
                u32::from_le_bytes(entry.try_into().or(Err(Error::SectionParseError))?)
            } as usize;
            e.sections = Vec::with_capacity(e.header.shnum as usize);
            for x in 1..e.header.shnum {
                let start = ((x * e.header.shentsize) + e.header.shoff as u16) as usize;

                let content = &executable[start + 4..start + e.header.shentsize as usize];
                let hdr = SectionHeader::parse(content)?;

                let header_name_offset = names_offset + {
                    let entry = &executable[start..start + 4];
                    u32::from_le_bytes(entry.try_into().or(Err(Error::SectionParseError))?)
                } as usize;

                let section = Section::parse(hdr, &executable[header_name_offset..]);
                if section.is_ok() {
                    e.sections.push(section?);
                } else if let Err(SectionNameError::Unknown) = section {
                    continue;
                }
            }
        }

        Ok(e)
    }
}
