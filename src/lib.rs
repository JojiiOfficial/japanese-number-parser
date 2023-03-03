use number_checker::{get_number_type, FormatType};
use parser::{
    financial::parse_financial, fractional::parse_fractional, positional::parse_positional,
    spelled_out::parse_spelled_out,
};

mod number_checker;
mod number_parts;
mod parser;

#[derive(Debug, PartialEq)]
pub enum VeryLargeNumberHandling {
    Regular,
    Alternate,
}

pub enum FractionalHandling {
    Bu,
    Wari,
}

pub struct JapaneseNumberFormatter {
    very_large_number_handling: VeryLargeNumberHandling,
    fractional_handling: FractionalHandling,
}

impl JapaneseNumberFormatter {
    #[inline]
    pub fn new() -> Self {
        JapaneseNumberFormatter {
            very_large_number_handling: VeryLargeNumberHandling::Regular,
            fractional_handling: FractionalHandling::Bu,
        }
    }

    pub fn very_large_number_handling(&mut self, handling: VeryLargeNumberHandling) -> &mut Self {
        self.very_large_number_handling = handling;
        self
    }

    pub fn fractional_handling(&mut self, handling: FractionalHandling) -> &mut Self {
        self.fractional_handling = handling;
        self
    }

    pub fn format(&self, japanese: &str) -> Option<String> {
        let number_type = get_number_type(japanese);
        println!("{:?}", number_type);

        let res = match number_type {
            Some(number_type) => match number_type {
                FormatType::Positional => {
                    parse_positional(japanese, &self.very_large_number_handling)
                }
                FormatType::SpelledOut => {
                    parse_spelled_out(japanese, &self.very_large_number_handling)
                }
                FormatType::Fractional => parse_fractional(japanese, &self.fractional_handling),
                FormatType::Financial => parse_financial(japanese),
            },
            None => return None,
        };

        Some(res)
    }
}
