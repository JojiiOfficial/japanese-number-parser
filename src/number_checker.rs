use itertools::Itertools;

mod financial;
mod fractional;
mod positional;
mod spelled_out;

use crate::{
    number_parts::{ALTERNATE_LARGE_POWERS, DECIMAL_POINTS, DIGITS, SEPARATORS, SEPARATOR_POWERS},
    parser::decimal::{has_decimal_separator, is_decimal_part_valid},
    VeryLargeNumberHandling,
};

use self::{
    financial::is_valid_financial, fractional::is_valid_fractional,
    positional::is_valid_japanese_positional, spelled_out::is_valid_spelled_out,
};

#[derive(Debug, PartialEq)]
pub enum FormatType {
    Positional,
    SpelledOut,
    Fractional,
    Financial,
}

pub fn get_number_type(japanese: &str) -> Option<FormatType> {
    let mut whole = japanese;
    if has_decimal_separator(japanese) {
        let parts = japanese.split(DECIMAL_POINTS).collect_vec();
        if parts.len() != 2 {
            return None;
        }
        whole = parts[0];
        if !is_decimal_part_valid(parts[1]) {
            return None;
        }
    } else {
        if is_valid_fractional(japanese) {
            return Some(FormatType::Fractional);
        }
    }

    if whole.chars().any(|c| SEPARATORS.contains(&c)) {
        if !is_separated_with_commas_properly(whole) {
            return None;
        }
    }

    if whole
        .chars()
        .all(|c| DIGITS.contains_key(&c) || SEPARATORS.contains(&c))
    {
        return Some(FormatType::Positional);
    }

    if is_valid_japanese_positional(whole) {
        return Some(FormatType::Positional);
    }

    if is_valid_spelled_out(whole) {
        return Some(FormatType::SpelledOut);
    }

    if is_valid_financial(whole) {
        return Some(FormatType::Financial);
    }

    None
}

fn is_separated_with_commas_properly(japanese: &str) -> bool {
    let mut chars = japanese.chars().rev();
    let mut count = 0;
    while let Some(c) = chars.next() {
        if SEPARATORS.contains(&c) {
            if count != 3 {
                return false;
            }
            count = 0;
        } else {
            count += 1;
        }
    }
    true
}

pub fn get_separator_value<I: Iterator<Item = char>>(
    iter: &mut I,
    very_large_number_handling: &VeryLargeNumberHandling,
) -> Option<u32> {
    let c = iter.next()?;
    let mut separator = c.to_string();
    for _ in 1..=4 {
        if let Some(sep_power) = SEPARATOR_POWERS.get(&separator.as_str()) {
            let power = if *very_large_number_handling == VeryLargeNumberHandling::Alternate
                && ALTERNATE_LARGE_POWERS.contains_key(&separator.as_str())
            {
                ALTERNATE_LARGE_POWERS[&separator.as_str()]
            } else {
                *sep_power
            };
            return Some(power);
        }

        let c = iter.next()?;
        separator.insert(0, c);
    }

    None
}
