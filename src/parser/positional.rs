use itertools::Itertools;

use crate::{
    number_checker::{get_separator_value, has_decimal_separator},
    number_parts::{DECIMAL_POINTS, DIGITS, SEPARATORS},
};

use super::decimal::parse_decimal_portion;

pub fn parse_positional(japanese: &str) -> String {
    let mut whole = japanese;
    let mut decimal = "".to_string();
    if has_decimal_separator(japanese) {
        let parts = japanese
            .split(|c| DECIMAL_POINTS.contains(&c))
            .collect_vec();
        if parts.len() != 2 {
            return String::new();
        }
        whole = parts[0];
        decimal = ".".to_string() + &parse_decimal_portion(parts[1]);
    }

    let mut chars = whole.chars().rev().peekable();
    let mut result = Vec::new();

    while let Some(c) = chars.peek() {
        if DIGITS.contains_key(&c) {
            let digit = DIGITS.get(&c).unwrap();
            result.push(digit.to_string());
            chars.next();
            continue;
        }
        if SEPARATORS.contains(&c) {
            chars.next();
            continue;
        }

        let power = get_separator_value(&mut chars).unwrap();
        result.resize(power as usize, "0".to_string());
    }

    result.iter().rev().join("") + decimal.as_str()
}
