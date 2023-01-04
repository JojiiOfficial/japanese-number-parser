use crate::number_parts::{DECIMAL_POINTS, DIGITS};

pub fn break_up_word(japanese: &str) -> (String, String) {
    if !has_decimal_separator(japanese) {
        return (japanese.to_string(), "".to_string());
    }

    let mut parts = japanese.split(|c| DECIMAL_POINTS.contains(&c));
    let (w, dc) = match (parts.next(), parts.next(), parts.next()) {
        (Some(whole), Some(dc), None) => (whole, dc),
        _ => return ("".to_string(), "".to_string()),
    };

    let decimal = format!(".{}", parse_decimal_portion(dc));
    return (w.to_string(), decimal.to_string());
}

fn parse_decimal_portion(decimal: &str) -> String {
    decimal.chars().map(|c| DIGITS.get(&c).unwrap()).collect()
}

pub fn has_decimal_separator(japanese: &str) -> bool {
    japanese.matches(|c| DECIMAL_POINTS.contains(&c)).count() == 1
}

pub fn is_decimal_part_valid(decimal: &str) -> bool {
    decimal.chars().all(|c| DIGITS.contains_key(&c))
}
