#[derive(Default, Debug)]
pub(crate) struct ELFHeader {
    entry: u32,

    phoff: u32,
    phentsize: u16,
    phnum: u16,

    pub(crate) shoff: u32,
    pub(crate) shentsize: u16,
    pub(crate) shnum: u16,
    pub(crate) shstrndx: u16,
}

impl ELFHeader {
    pub(crate) fn is_valid(executable: &[u8]) -> Result<(), ELFHeaderError> {
        // magic
        if let &[0x7F, 0x45, 0x4C, 0x46] = &executable[0..4] {
            // 32-bit
            if let &[1] = &executable[4..5] {
                // risc
                if let &[0xF3] = &executable[0x12..0x13] {
                    return Ok(());
                } else {
                    return Err(ELFHeaderError::InvalidISA);
                }
            } else {
                return Err(ELFHeaderError::InvalidClassFormat);
            }
        } else {
            return Err(ELFHeaderError::InvalidMagic);
        }
    }

    pub(crate) fn parse(executable: &[u8]) -> Result<Self, ELFHeaderError> {
        Self::is_valid(executable)?;
        let mut e: Self = Default::default();

        e.entry = {
            let entry = &executable[0x18..0x18 + 4];
            u32::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidEntryPoint))?,
            )
        };

        e.phoff = {
            let entry = &executable[0x1C..0x1C + 4];
            u32::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidProgramHeaderOffset))?,
            )
        };
        e.phentsize = {
            let entry = &executable[0x2A..0x2A + 2];
            u16::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidProgramHeaderSize))?,
            )
        };
        e.phnum = {
            let entry = &executable[0x2C..0x2C + 2];
            u16::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidProgramHeaderEntityCount))?,
            )
        };

        e.shoff = {
            let entry = &executable[0x20..0x20 + 4];
            u32::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidSectionHeaderOffset))?,
            )
        };
        e.shentsize = {
            let entry = &executable[0x2E..0x2E + 2];
            u16::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidSectionHeaderSize))?,
            )
        };
        e.shnum = {
            let entry = &executable[0x30..0x30 + 2];
            u16::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidSectionHeaderEntityCount))?,
            )
        };
        e.shstrndx = {
            let entry = &executable[0x32..0x32 + 2];
            u16::from_le_bytes(
                entry
                    .try_into()
                    .or(Err(ELFHeaderError::InvalidSectionHeaderNamesOffset))?,
            )
        };

        Ok(e)
    }
}

#[derive(Debug)]
pub(crate) enum ELFHeaderError {
    InvalidMagic,
    InvalidClassFormat,
    InvalidISA,

    InvalidEntryPoint,

    InvalidProgramHeaderOffset,
    InvalidProgramHeaderSize,
    InvalidProgramHeaderEntityCount,

    InvalidSectionHeaderOffset,
    InvalidSectionHeaderSize,
    InvalidSectionHeaderEntityCount,
    InvalidSectionHeaderNamesOffset,
}
