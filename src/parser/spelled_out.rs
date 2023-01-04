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
    let mut decimal = "".to_string();
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
        if DIGITS.contains_key(&c) {
            let digit = DIGITS.get(&c).unwrap();
            result.push(digit.to_string());
            current_group_length += 1;
            chars.next();
            previous_digit = true;
            continue;
        }
        if IN_GROUP_POWERS.contains_key(&c) {
            let power = IN_GROUP_POWERS.get(&c).unwrap();

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
