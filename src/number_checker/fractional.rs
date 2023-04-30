use itertools::Itertools;

use crate::{
    number_parts::{BU_FRACTIONALS, DIGITS, WARI_FRACTIONALS},
    FractionalHandling,
};

pub fn is_valid_fractional(japanese: &str, handling: &FractionalHandling) -> bool {
    if japanese.chars().count() % 2 != 0 {
        return false;
    }

    if japanese.chars().chunks(2).into_iter().any(|chunk| {
        let mut chunk = chunk.into_iter();
        let first = chunk.next().unwrap();
        let second = chunk.next().unwrap();
        let is_second_fractional = match handling {
            FractionalHandling::Bu => BU_FRACTIONALS.contains_key(&second),
            FractionalHandling::Wari => WARI_FRACTIONALS.contains_key(&second),
        };
        !DIGITS.contains_key(&first) || !is_second_fractional
    }) {
        return false;
    }

    if japanese
        .chars()
        .filter(|c| WARI_FRACTIONALS.contains_key(&c))
        .tuple_windows()
        .any(|(a, b)| WARI_FRACTIONALS.get(&a) > WARI_FRACTIONALS.get(&b))
    {
        return false;
    }
    true
}
