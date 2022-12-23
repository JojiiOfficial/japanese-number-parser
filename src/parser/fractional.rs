use itertools::Itertools;

use crate::{
    number_parts::{BU_FRACTIONALS, DIGITS, WARI_FRACTIONALS},
    FractionalHandling,
};

pub fn parse_fractional(japanese: &str, handling: &FractionalHandling) -> String {
    let mut result = Vec::new();

    let get_separator = |c: &char| match handling {
        FractionalHandling::Bu => BU_FRACTIONALS.get(&c).unwrap(),
        FractionalHandling::Wari => WARI_FRACTIONALS.get(&c).unwrap(),
    };

    let chunks = japanese.chars().chunks(2);
    let mut chunks_iter = chunks.into_iter();

    while let Some(chunk) = chunks_iter.next() {
        let mut chunk = chunk.into_iter();
        let first = chunk.next().unwrap();
        let second = chunk.next().unwrap();
        let digit = DIGITS.get(&first).unwrap();
        let separator = get_separator(&second);
        result.resize(*separator as usize - 1, "0".to_string());
        result.push(digit.to_string());
    }

    "0.".to_owned() + &result.join("")
}
