pub trait BinaryFormattable: std::fmt::Binary + std::fmt::Display + Copy {
    fn format(&self, width: usize) -> String;
}

impl BinaryFormattable for i32 {
    fn format(&self, width: usize) -> String {
        format!(
            "0b{}",
            format!("{:0width$b}", self, width = width)
                .as_bytes()
                .rchunks(4)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join("_")
        )
    }
}

impl BinaryFormattable for u32 {
    fn format(&self, width: usize) -> String {
        (*self as i32).format(width)
    }
}

pub fn binary<T: BinaryFormattable>(num: T, width: usize) -> String {
    num.format(width)
}

pub fn number<T: BinaryFormattable>(num: T, width: usize) -> String {
    format!("{} ({})", binary(num, width), num)
}
