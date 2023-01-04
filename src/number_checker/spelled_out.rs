use crate::{
    number_parts::{DIGITS, IN_GROUP_POWERS},
    VeryLargeNumberHandling,
};

use super::get_separator_value;

#[derive(Debug, PartialEq)]
enum SpelledOutState {
    GroupStart,
    Digit,
    GroupSeparator,
}

pub fn is_valid_spelled_out(japanese: &str) -> bool {
    let mut state = SpelledOutState::GroupStart;
    let mut last_power = 0;
    let mut last_group_power = 0;
    let mut chars = japanese.chars().rev().peekable();
    while let Some(c) = chars.peek() {
        match state {
            SpelledOutState::GroupStart => {
                if DIGITS.contains_key(&c) {
                    state = SpelledOutState::Digit;
                    chars.next();
                } else if let Some(power) = IN_GROUP_POWERS.get(&c) {
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
                }
                if let Some(power) = IN_GROUP_POWERS.get(&c) {
                    if *power < last_group_power {
                        return false;
                    }
                    last_group_power = *power;
                    state = SpelledOutState::GroupSeparator;
                    chars.next();
                } else {
                    let power =
                        match get_separator_value(&mut chars, &VeryLargeNumberHandling::Regular) {
                            Some(p) => p,
                            None => return false,
                        };
                    if power < last_power {
                        return false;
                    }
                    last_power = power;
                    state = SpelledOutState::GroupStart;
                    last_group_power = 0;
                }
            }
            SpelledOutState::GroupSeparator => {
                if DIGITS.contains_key(&c) {
                    state = SpelledOutState::Digit;
                    chars.next();
                } else if IN_GROUP_POWERS.contains_key(&c) {
                    state = SpelledOutState::GroupSeparator;
                    chars.next();
                } else {
                    let power =
                        match get_separator_value(&mut chars, &VeryLargeNumberHandling::Regular) {
                            Some(p) => p,
                            None => return false,
                        };
                    if power < last_power {
                        return false;
                    }
                    last_power = power;
                    state = SpelledOutState::GroupStart;
                    last_group_power = 0;
                }
            }
        }
    }

    true
}
