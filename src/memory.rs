#[derive(Debug)]
pub struct Memory {
    data: Box<[u8]>,
}

impl Memory {
    pub(crate) fn new(pool: u32) -> Self {
        Self {
            data: vec![0; pool as usize].into_boxed_slice(),
        }
    }

    pub fn byte_at(&self, address: u32) -> Result<u8, &'static str> {
        let address = address as usize;
        if address >= self.data.len() {
            return Err("invalid address");
        }
        Ok(self.data[address])
    }

    pub fn set_byte_at(&mut self, address: u32, b: u8) -> Result<(), &'static str> {
        let address = address as usize;
        if address >= self.data.len() {
            return Err("invalid address");
        }
        self.data[address] = b;
        Ok(())
    }

    pub fn hw_at(&self, address: u32) -> Result<u16, &'static str> {
        let address = address as usize;
        if address + 1 >= self.data.len() {
            return Err("invalid address");
        }
        let b1 = self.data[address] as u16;
        let b2 = (self.data[address + 1] as u16) << 8;
        let res = b1 | b2;
        // eprintln!("get m1: {:#018b} ({})", self.data[address], self.data[address]);
        // eprintln!("get m2: {:#018b} ({})", self.data[address+1], self.data[address+1]);
        // eprintln!("get 1: {:#018b} ({})", b1, b1);
        // eprintln!("get 2: {:#018b} ({})", b2, b2);
        // eprintln!("get e: {:#018b} ({})", res, res);
        Ok(res)
    }

    #[allow(clippy::identity_op)] // readability
    pub fn set_hw_at(&mut self, address: u32, hw: u16) -> Result<(), &'static str> {
        let address = address as usize;
        if address + 1 >= self.data.len() {
            return Err("invalid address");
        }
        let b1 = hw as u8;
        let b2 = (hw >> 8) as u8;
        self.data[address + 0] = b1;
        self.data[address + 1] = b2;
        // let res = b1 as u16 | ((b2 as u16) << 8);
        // eprintln!("set v: {:#018b} ({})", hw, hw);
        // eprintln!("set 1: {:#010b} ({})", b1, b1);
        // eprintln!("set 2: {:#010b} ({})", b2, b2);
        // eprintln!("set e: {:#018b} ({})", res, res);
        // eprintln!("set m1: {:#018b} ({})", self.data[address], self.data[address]);
        // eprintln!("set m2: {:#018b} ({})", self.data[address+1], self.data[address+1]);
        Ok(())
    }

    #[allow(clippy::identity_op)] // readability
    pub fn word_at(&self, address: u32) -> Result<u32, &'static str> {
        let address = address as usize;
        if address + 3 >= self.data.len() {
            return Err("invalid address");
        }
        let b1 = (self.data[address + 0] as u32) << 0;
        let b2 = (self.data[address + 1] as u32) << 8;
        let b3 = (self.data[address + 2] as u32) << 16;
        let b4 = (self.data[address + 3] as u32) << 24;
        let res = b1 | b2 | b3 | b4;
        // eprintln!("get m1: {:#010b} ({})", self.data[address+0], self.data[address]);
        // eprintln!("get m2: {:#010b} ({})", self.data[address+1], self.data[address+1]);
        // eprintln!("get m3: {:#010b} ({})", self.data[address+2], self.data[address+2]);
        // eprintln!("get m4: {:#010b} ({})", self.data[address+3], self.data[address+3]);
        // eprintln!("get 1:  {:#010b} ({})", b1, b1);
        // eprintln!("get 2:  {:#010b} ({})", b2, b2);
        // eprintln!("get 3:  {:#010b} ({})", b3, b3);
        // eprintln!("get 4:  {:#010b} ({})", b4, b4);
        // eprintln!("get e:  {:#034b} ({})", res, res);
        Ok(res)
    }

    #[allow(clippy::identity_op)] // readability
    pub fn set_word_at(&mut self, address: u32, hw: u32) -> Result<(), &'static str> {
        let address = address as usize;
        if address + 3 >= self.data.len() {
            return Err("invalid address");
        }
        let b1 = (hw >> 0) as u8;
        let b2 = (hw >> 8) as u8;
        let b3 = (hw >> 16) as u8;
        let b4 = (hw >> 24) as u8;
        self.data[address + 0] = b1;
        self.data[address + 1] = b2;
        self.data[address + 2] = b3;
        self.data[address + 3] = b4;
        // let res = b1 as u32 | ((b2 as u32) << 8) | ((b3 as u32) << 16) | ((b4 as u32) << 24);
        // eprintln!("set v:  {:#018b} ({})", hw, hw);
        // eprintln!("set 1:  {:#010b} ({})", b1, b1);
        // eprintln!("set 2:  {:#010b} ({})", b2, b2);
        // eprintln!("set 3:  {:#010b} ({})", b3, b3);
        // eprintln!("set 4:  {:#010b} ({})", b4, b4);
        // eprintln!("set e:  {:#034b} ({})", res, res);
        // eprintln!("set m1: {:#018b} ({})", self.data[address+0], self.data[address]);
        // eprintln!("set m2: {:#018b} ({})", self.data[address+1], self.data[address+1]);
        // eprintln!("set m3: {:#018b} ({})", self.data[address+2], self.data[address+2]);
        // eprintln!("set m4: {:#018b} ({})", self.data[address+3], self.data[address+3]);
        Ok(())
    }
}

const DEFAULT_MEMORY_POOL_SIZE: u32 = 1024;
impl Default for Memory {
    fn default() -> Self {
        Self::new(DEFAULT_MEMORY_POOL_SIZE)
    }
}

#[cfg(test)]
mod byte {
    use super::*;

    #[test]
    fn memory_access_violation_get() {
        let m = Memory::new(12);
        if m.byte_at(13).is_ok() {
            assert!(false, "expected error");
        }
    }

    #[test]
    fn memory_access_violation_set() {
        let mut m = Memory::new(12);
        if m.set_byte_at(13, 1).is_ok() {
            assert!(false, "expected error");
        }
    }

    #[test]
    fn happy_path() {
        let mut m = Memory::new(13);
        m.set_byte_at(12, 161).unwrap();
        assert_eq!(m.byte_at(12).unwrap(), 161);
    }
}

#[cfg(test)]
mod hw {
    use super::*;

    #[test]
    fn memory_access_violation_get() {
        let m = Memory::new(12);
        if m.hw_at(12).is_ok() {
            assert!(false, "expected error");
        }
    }

    #[test]
    fn memory_access_violation_set() {
        let mut m = Memory::new(12);
        if m.set_hw_at(12, 1).is_ok() {
            assert!(false, "expected error");
        }
    }

    #[test]
    fn happy_path() {
        let mut m = Memory::new(13);
        m.set_hw_at(11, 1312).unwrap();
        assert_eq!(m.hw_at(11).unwrap(), 1312);
    }

    #[test]
    fn happy_path_large_number() {
        let mut m = Memory::new(13);
        m.set_hw_at(11, 65535).unwrap();
        assert_eq!(m.hw_at(11).unwrap(), 65535);
    }
}

#[cfg(test)]
mod word {
    use super::*;

    #[test]
    fn memory_access_violation_get() {
        let m = Memory::new(12);
        if m.word_at(12).is_ok() {
            assert!(false, "expected error");
        }
    }

    #[test]
    fn memory_access_violation_set() {
        let mut m = Memory::new(12);
        if m.set_word_at(12, 1).is_ok() {
            assert!(false, "expected error");
        }
    }

    #[test]
    fn happy_path() {
        let mut m = Memory::new(13);
        m.set_word_at(8, 1312).unwrap();
        assert_eq!(m.word_at(8).unwrap(), 1312);
    }

    #[test]
    fn happy_path_large_number() {
        let mut m = Memory::new(13);
        m.set_word_at(8, 4294967295).unwrap();
        assert_eq!(m.word_at(8).unwrap(), 4294967295);
    }
}
