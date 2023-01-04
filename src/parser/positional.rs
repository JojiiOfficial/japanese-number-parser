use itertools::Itertools;

use crate::{
    number_checker::{get_separator_value, has_decimal_separator},
    number_parts::{DECIMAL_POINTS, DIGITS, SEPARATORS},
    VeryLargeNumberHandling,
};

use super::decimal::parse_decimal_portion;

pub fn parse_positional(
    japanese: &str,
    very_large_number_handling: &VeryLargeNumberHandling,
) -> String {
    let mut whole = japanese;
    let mut decimal = String::new();
    if has_decimal_separator(japanese) {
        let mut parts = japanese.split(|c| DECIMAL_POINTS.contains(&c));

        let (w, dc) = match (parts.next(), parts.next(), parts.next()) {
            (Some(whole), Some(dc), None) => (whole, dc),
            _ => return String::new(),
        };
        whole = w;

        decimal = format!(".{}", parse_decimal_portion(dc))
    }

    let mut chars = whole.chars().rev().peekable();
    let mut result = Vec::new();

    while let Some(c) = chars.peek() {
        if let Some(digit) = DIGITS.get(&c) {
            result.push(digit.to_string());
            chars.next();
            continue;
        }

        if SEPARATORS.contains(&c) {
            chars.next();
            continue;
        }

        let power = get_separator_value(&mut chars, very_large_number_handling).unwrap();
        result.resize(power as usize, "0".to_string());
    }

    result.iter().rev().join("") + decimal.as_str()
}
