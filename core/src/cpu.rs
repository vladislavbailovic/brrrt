#[derive(Default, Debug)]
pub struct CPU {
    pub register: Registers,
}

impl CPU {
    pub fn increment_pc(&mut self) {
        self.register.set(
            Register::PC,
            self.register.get(Register::PC) + REGISTER_INCREMENT,
        );
    }
}

pub const REGISTER_INCREMENT: u32 = 4;

#[derive(Debug)]
pub struct Registers {
    data: [u32; 33],
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            data: [(); 33].map(|_| 0),
        }
    }
}

impl Registers {
    pub fn set(&mut self, key: Register, value: u32) {
        self.data[key as usize] = value;
    }

    pub fn get(&self, key: Register) -> u32 {
        self.data[key as usize]
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Register {
    X0,
    X1, // return address of a call
    X2, // stack pointer
    X3,
    X4,
    X5, // available as an alternate link register
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,

    PC,
}

impl TryFrom<u32> for Register {
    type Error = &'static str;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            x if x == Self::X0 as u32 => Ok(Self::X0),
            x if x == Self::X1 as u32 => Ok(Self::X1),
            x if x == Self::X2 as u32 => Ok(Self::X2),
            x if x == Self::X3 as u32 => Ok(Self::X3),
            x if x == Self::X4 as u32 => Ok(Self::X4),
            x if x == Self::X5 as u32 => Ok(Self::X5),
            x if x == Self::X6 as u32 => Ok(Self::X6),
            x if x == Self::X7 as u32 => Ok(Self::X7),
            x if x == Self::X8 as u32 => Ok(Self::X8),
            x if x == Self::X9 as u32 => Ok(Self::X9),
            x if x == Self::X10 as u32 => Ok(Self::X10),
            x if x == Self::X11 as u32 => Ok(Self::X11),
            x if x == Self::X12 as u32 => Ok(Self::X12),
            x if x == Self::X13 as u32 => Ok(Self::X13),
            x if x == Self::X14 as u32 => Ok(Self::X14),
            x if x == Self::X15 as u32 => Ok(Self::X15),
            x if x == Self::X16 as u32 => Ok(Self::X16),
            x if x == Self::X17 as u32 => Ok(Self::X17),
            x if x == Self::X18 as u32 => Ok(Self::X18),
            x if x == Self::X19 as u32 => Ok(Self::X19),
            x if x == Self::X20 as u32 => Ok(Self::X20),
            x if x == Self::X21 as u32 => Ok(Self::X21),
            x if x == Self::X22 as u32 => Ok(Self::X22),
            x if x == Self::X23 as u32 => Ok(Self::X23),
            x if x == Self::X24 as u32 => Ok(Self::X24),
            x if x == Self::X25 as u32 => Ok(Self::X25),
            x if x == Self::X26 as u32 => Ok(Self::X26),
            x if x == Self::X27 as u32 => Ok(Self::X27),
            x if x == Self::X28 as u32 => Ok(Self::X28),
            x if x == Self::X29 as u32 => Ok(Self::X29),
            x if x == Self::X30 as u32 => Ok(Self::X30),

            x if x == Self::PC as u32 => Ok(Self::PC),

            _ => Err("unknown register"),
        }
    }
}

impl TryFrom<String> for Register {
    type Error = &'static str;

    fn try_from(regname: String) -> Result<Self, Self::Error> {
        if "PC" == &regname {
            Ok(Register::PC)
        } else {
            let regname = regname.to_lowercase();
            let first = regname.chars().next();
            if Some('x') == first && regname.len() > 1 {
                regname
                    .strip_prefix('x')
                    .ok_or("invalid prefix")?
                    .parse::<u32>()
                    .map_err(|_: std::num::ParseIntError| "not a number")?
                    .try_into()
            } else {
                Err("invalid register")
            }
        }
    }
}

impl TryInto<String> for Register {
    type Error = ();

    fn try_into(self) -> Result<String, ()> {
        match self {
            Self::PC => Ok("PC".to_owned()),
            Self::X0 => Ok("X0".to_owned()),
            Self::X1 => Ok("X1".to_owned()),
            Self::X2 => Ok("X2".to_owned()),
            Self::X3 => Ok("X3".to_owned()),
            Self::X4 => Ok("X4".to_owned()),
            Self::X5 => Ok("X5".to_owned()),
            Self::X6 => Ok("X6".to_owned()),
            Self::X7 => Ok("X7".to_owned()),
            Self::X8 => Ok("X8".to_owned()),
            Self::X9 => Ok("X9".to_owned()),
            Self::X10 => Ok("X10".to_owned()),
            Self::X11 => Ok("X11".to_owned()),
            Self::X12 => Ok("X12".to_owned()),
            Self::X13 => Ok("X13".to_owned()),
            Self::X14 => Ok("X14".to_owned()),
            Self::X15 => Ok("X15".to_owned()),
            Self::X16 => Ok("X16".to_owned()),
            Self::X17 => Ok("X17".to_owned()),
            Self::X18 => Ok("X18".to_owned()),
            Self::X19 => Ok("X19".to_owned()),
            Self::X20 => Ok("X20".to_owned()),
            Self::X21 => Ok("X21".to_owned()),
            Self::X22 => Ok("X22".to_owned()),
            Self::X23 => Ok("X23".to_owned()),
            Self::X24 => Ok("X24".to_owned()),
            Self::X25 => Ok("X25".to_owned()),
            Self::X26 => Ok("X26".to_owned()),
            Self::X27 => Ok("X27".to_owned()),
            Self::X28 => Ok("X28".to_owned()),
            Self::X29 => Ok("X29".to_owned()),
            Self::X30 => Ok("X30".to_owned()),
            Self::X31 => Ok("X31".to_owned()),
        }
    }
}
