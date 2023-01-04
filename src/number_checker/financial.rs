use crate::number_parts::{DIGITS, FINANCIAL_SEPARATORS};

pub fn is_valid_financial(japanese: &str) -> bool {
    let mut chars = japanese.chars().peekable();
    let mut group_length = 0;
    let mut last_power = 0;
    let mut first = true;
    while let Some(c) = chars.next() {
        if DIGITS.contains_key(&c) {
            group_length += 1;
            if group_length > 3 {
                return false;
            }
            continue;
        }

        if chars.peek().is_some() {
            let potential_separator = format!("{}{}", c, chars.peek().unwrap());
            if let Some(power) = FINANCIAL_SEPARATORS.get(&potential_separator.as_str()) {
                if last_power > 0 && power > &last_power {
                    return false;
                }
                last_power = *power;
                group_length = 0;
                chars.next();
                continue;
            }
        }

        let potential_separator = c.to_string();
        if let Some(power) = FINANCIAL_SEPARATORS.get(&potential_separator.as_str()) {
            if !first && group_length != 3 {
                return false;
            }
            first = false;
            group_length = 0;

            if last_power > 0 && power > &last_power {
                return false;
            }

            last_power = *power;
            continue;
        }

        return false;
    }

    true
}
