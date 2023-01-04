use itertools::Itertools;

use crate::{
    number_checker::has_decimal_separator,
    number_parts::{DECIMAL_POINTS, DIGITS, FINANCIAL_SEPARATORS},
};

use super::decimal::parse_decimal_portion;

pub fn parse_financial(japanese: &str) -> String {
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

    while let Some(c) = chars.next() {
        if DIGITS.contains_key(&c) {
            let digit = DIGITS.get(&c).unwrap();
            result.push(digit.to_string());
            continue;
        }

        let next_char = chars.peek();
        if next_char.is_some() {
            let potential_separator = format!("{}{}", next_char.unwrap(), c);
            if FINANCIAL_SEPARATORS.contains_key(&potential_separator.as_str()) {
                let power = FINANCIAL_SEPARATORS
                    .get(&potential_separator.as_str())
                    .unwrap();
                result.resize(*power as usize, "0".to_string());
                chars.next();
                continue;
            }
        }
        let potential_separator = format!("{}", c);
        if FINANCIAL_SEPARATORS.contains_key(&potential_separator.as_str()) {
            let power = FINANCIAL_SEPARATORS
                .get(&potential_separator.as_str())
                .unwrap();
            result.resize(*power as usize, "0".to_string());
            continue;
        }
    }

    result.iter().rev().join("") + decimal.as_str()
}
