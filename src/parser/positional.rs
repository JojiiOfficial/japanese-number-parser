use itertools::Itertools;

use crate::{
    number_checker::get_separator_value,
    number_parts::{DIGITS, SEPARATORS},
    VeryLargeNumberHandling,
};

use super::decimal::break_up_word;

pub fn parse_positional(
    japanese: &str,
    very_large_number_handling: &VeryLargeNumberHandling,
) -> String {
    let (whole, decimal) = break_up_word(japanese);

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
