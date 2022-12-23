use crate::number_parts::DIGITS;

pub fn parse_decimal_portion(decimal: &str) -> String {
    decimal.chars().map(|c| DIGITS.get(&c).unwrap()).collect()
}
