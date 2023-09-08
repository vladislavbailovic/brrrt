use super::operation::Operation;
use super::part::Part;

pub(crate) struct Builder {
    raw: u32,
}

impl Builder {
    pub(crate) fn new(raw: u32) -> Self {
        Self { raw }
    }

    pub(crate) fn opcode(code: Operation) -> Self {
        Self::new(code as u32 | Part::Opcode as u32)
    }

    pub(crate) fn pack(&self, part: Part, data: u32) -> Self {
        let packed = data << part.shift();
        Builder::new(self.raw | (packed & part.mask()))
    }

    pub fn build(&self) -> u32 {
        self.raw
    }
}
