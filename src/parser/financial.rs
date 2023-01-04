use itertools::Itertools;

use crate::{
    number_checker::has_decimal_separator,
    number_parts::{DECIMAL_POINTS, DIGITS, FINANCIAL_SEPARATORS},
};

use super::decimal::parse_decimal_portion;

pub fn parse_financial(japanese: &str) -> String {
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

    while let Some(c) = chars.next() {
        if let Some(digit) = DIGITS.get(&c) {
            result.push(digit.to_string());
            continue;
        }

        if let Some(next_char) = chars.peek() {
            let potential_separator = format!("{}{}", next_char, c);
            if let Some(power) = FINANCIAL_SEPARATORS.get(&potential_separator.as_str()) {
                result.resize(*power as usize, "0".to_string());
                chars.next();
                continue;
            }
        }

        let potential_separator = c.to_string();
        if let Some(power) = FINANCIAL_SEPARATORS.get(&potential_separator.as_str()) {
            result.resize(*power as usize, "0".to_string());
            //continue;
        }
    }

    result.iter().rev().join("") + decimal.as_str()
}
