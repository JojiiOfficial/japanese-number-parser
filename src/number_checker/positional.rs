use crate::{number_parts::{DIGITS, ZERO_DIGITS}, VeryLargeNumberHandling};

use super::get_separator_value;

pub fn is_valid_japanese_positional(japanese: &str) -> bool {
    let mut chars = japanese.chars().rev().peekable();
    let mut group = 0;
    let mut last_separator = 0;

    let mut last_digit_zero = false;

    while let Some(c) = chars.peek() {
        if DIGITS.contains_key(&c) {
            last_digit_zero = ZERO_DIGITS.contains(c);
            chars.next();
            group += 1;
            continue;
        }
        if group != 4 && (last_separator != 0 || group != 0) {
            return false;
        }
        if last_digit_zero {
            return false;
        }
        group = 0;
        let power = match get_separator_value(&mut chars, &VeryLargeNumberHandling::Regular) {
            Some(power) => power,
            None => return false,
        };
        if power < last_separator {
            return false;
        }
        last_separator = power;
    }

    group != 0 && !last_digit_zero
}
