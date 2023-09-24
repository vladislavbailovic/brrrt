pub fn first_lsb_set(source: u32) -> i32 {
    for x in 0..32 {
        if source & (1 << x) > 0 {
            return x;
        }
    }

    -1
}

#[cfg(test)]
mod test_first_lsb_set {
    use super::*;
    use crate::risc32i::instr::part::Part;

    #[test]
    fn lsb() {
        let test = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let result = first_lsb_set(test);
        assert_eq!(result, -1);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        let result = first_lsb_set(test);
        assert_eq!(result, 0);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        let result = first_lsb_set(test);
        assert_eq!(result, 1);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        let result = first_lsb_set(test);
        assert_eq!(result, 2);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        let result = first_lsb_set(test);
        assert_eq!(result, 3);

        let test = 0b1000_0000_0000_0000_0000_0000_0000_1000;
        let result = first_lsb_set(test);
        assert_eq!(result, 3);
    }

    #[test]
    fn parts() {
        let parts = vec![
            // Part::Null,
            Part::Opcode,
            Part::Dest,
            Part::Reg1,
            Part::Reg2,
            Part::Funct3,
            Part::Funct7,
            Part::Imm110,
            Part::Imm3112,
            Part::Imm40,
            Part::Imm41,
            Part::Imm105,
            Part::Imm115,
            Part::Imm1912,
            Part::Imm101,
            Part::B11b,
            Part::B12b,
            Part::B11j,
            Part::B20j,
        ];
        for part in parts {
            let result = first_lsb_set(part.mask());
            assert_eq!(result, part.shift() as i32, "{:?}", part);
        }
    }
}

pub fn sign_extend(v: u32, width: u32) -> i32 {
    assert!(width < 32);
    let base: i32 = 2;
    let mut res = v as i32;
    if v as i32 > base.pow(width - 1) {
        res = v as i32 - base.pow(width);
    }
    res
}

#[cfg(test)]
mod test_sign_extend {
    use super::*;

    #[test]
    fn extend_neg_one() {
        let neg = -1;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i8, neg as i8);

        let neg = -13;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i8, neg as i8);

        let neg = -161;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i16, neg as i16);

        let neg = -1312;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i16, neg as i16);
    }
}
