use std::{
    iter::{Peekable, Rev},
    str::Chars,
};

use itertools::Itertools;

use crate::number_parts::{
    DECIMAL_POINTS, DIGITS, IN_GROUP_POWERS, SEPARATORS, SEPARATOR_POWERS, WARI_FRACTIONALS,
};

pub enum FormatType {
    Positional,
    SpelledOut,
    Fractional,
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

    // It's fractional, if it's alternating digits with WARI_SEPARATORS
    if whole.len() % 2 == 0
        && whole.chars().chunks(2).into_iter().all(|chunk| {
            let mut chunk = chunk.into_iter();
            let first = chunk.next().unwrap();
            let second = chunk.next().unwrap();
            DIGITS.contains_key(&first) && WARI_FRACTIONALS.contains_key(&second)
        })
    {
        return Some(FormatType::Fractional);
    }

    if is_valid_spelled_out(whole) {
        return Some(FormatType::SpelledOut);
    }

    None
}

pub fn has_decimal_separator(japanese: &str) -> bool {
    japanese.matches(|c| DECIMAL_POINTS.contains(&c)).count() == 1
}

fn is_decimal_part_valid(decimal: &str) -> bool {
    decimal.chars().all(|c| DIGITS.contains_key(&c))
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

fn is_valid_japanese_positional(japanese: &str) -> bool {
    let mut chars = japanese.chars().rev().peekable();
    let mut group = 0;
    let mut last_separator = 0;
    let mut first = true;

    while let Some(c) = chars.peek() {
        if DIGITS.contains_key(&c) {
            chars.next();
            group += 1;
            continue;
        }
        if group != 4 && (!first || group != 0) {
            return false;
        }
        first = false;
        group = 0;
        let power = get_separator_value(&mut chars);
        if power.is_none() {
            return false;
        }
        if power.unwrap() < last_separator {
            return false;
        }
        last_separator = power.unwrap();
    }
    true
}

pub fn get_separator_value(iter: &mut Peekable<Rev<Chars>>) -> Option<u32> {
    if iter.peek().is_none() {
        return None;
    }
    let c = iter.next().unwrap();
    let mut separator = c.to_string();
    for _ in 1..=4 {
        if SEPARATOR_POWERS.contains_key(&separator.as_str()) {
            let power = SEPARATOR_POWERS.get(&separator.as_str()).unwrap();
            return Some(*power);
        }
        if let Some(c) = iter.next() {
            separator.insert(0, c);
        } else {
            return None;
        }
    }
    None
}

enum SpelledOutState {
    GroupStart,
    Digit,
    GroupSeparator,
}

fn is_valid_spelled_out(japanese: &str) -> bool {
    let mut state = SpelledOutState::GroupStart;
    let mut last_power = 0;
    let mut last_group_power = 0;
    let mut chars = japanese.chars().rev().peekable();
    while let Some(c) = chars.peek() {
        match state {
            SpelledOutState::GroupStart => {
                if DIGITS.contains_key(&c) {
                    state = SpelledOutState::Digit;
                } else if IN_GROUP_POWERS.contains_key(&c) {
                    let power = IN_GROUP_POWERS.get(&c).unwrap();
                    if *power < last_group_power {
                        return false;
                    }
                    last_group_power = *power;
                    state = SpelledOutState::GroupSeparator;
                } else {
                    return false;
                }
            }
            SpelledOutState::Digit => {
                if DIGITS.contains_key(&c) {
                    return false;
                } else if IN_GROUP_POWERS.contains_key(&c) {
                    let power = IN_GROUP_POWERS.get(&c).unwrap();
                    if *power < last_group_power {
                        return false;
                    }
                    last_group_power = *power;
                    state = SpelledOutState::GroupSeparator;
                } else {
                    let power = get_separator_value(&mut chars);
                    if power.is_none() {
                        return false;
                    }
                    if power.unwrap() < last_power {
                        return false;
                    }
                    last_power = power.unwrap();
                    state = SpelledOutState::GroupStart;
                    last_group_power = 0;
                }
            }
            SpelledOutState::GroupSeparator => {
                if DIGITS.contains_key(&c) {
                    state = SpelledOutState::Digit;
                } else if IN_GROUP_POWERS.contains_key(&c) {
                    state = SpelledOutState::GroupSeparator;
                } else {
                    let power = get_separator_value(&mut chars);
                    if power.is_none() {
                        return false;
                    }
                    if power.unwrap() < last_power {
                        return false;
                    }
                    last_power = power.unwrap();
                    state = SpelledOutState::GroupStart;
                    last_group_power = 0;
                }
            }
        }
        chars.next();
    }
    true
}
