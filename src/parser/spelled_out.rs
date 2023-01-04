use itertools::Itertools;

use crate::{
    number_checker::{get_separator_value, has_decimal_separator},
    number_parts::{DECIMAL_POINTS, DIGITS, IN_GROUP_POWERS},
    VeryLargeNumberHandling,
};

use super::decimal::parse_decimal_portion;

pub fn parse_spelled_out(
    japanese: &str,
    very_large_number_handling: &VeryLargeNumberHandling,
) -> String {
    let mut whole = japanese;
    let mut decimal = String::new();

    if has_decimal_separator(japanese) {
        let parts = whole.split(|c| DECIMAL_POINTS.contains(&c)).collect_vec();
        if parts.len() != 2 {
            return String::new();
        }

        whole = parts[0];
        decimal = ".".to_string() + &parse_decimal_portion(parts[1]);
    }

    let mut result = Vec::new();

    let mut chars = whole.chars().rev().peekable();
    let mut current_group_length = 0;
    let mut previous_digit = true;
    while let Some(c) = chars.peek() {
        if let Some(digit) = DIGITS.get(&c) {
            result.push(digit.to_string());
            current_group_length += 1;
            chars.next();
            previous_digit = true;
            continue;
        }

        if let Some(power) = IN_GROUP_POWERS.get(&c) {
            if !previous_digit {
                result.push("1".to_string());
                current_group_length += 1;
            }
            previous_digit = false;

            while power > &current_group_length {
                result.push("0".to_string());
                current_group_length += 1;
            }

            chars.next();
            if chars.peek().is_none() {
                result.push("1".to_string());
            }
            continue;
        }

        let power = get_separator_value(&mut chars, very_large_number_handling).unwrap();
        result.resize(power as usize, "0".to_string());
        current_group_length = 0;
    }

    result.iter().rev().join("") + &decimal
}
