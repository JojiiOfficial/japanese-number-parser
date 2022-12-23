#[cfg(test)]
mod fractionals {
    use japanese_number_parser::{FractionalHandling, JapaneseNumberFormatter};

    #[test]
    fn bu_fractionals() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(formatter.format("一分二厘"), Some("0.12".to_string()));
    }

    #[test]
    fn wari_fractionals() {
        let mut formatter = JapaneseNumberFormatter::new();
        formatter.fractional_handling(FractionalHandling::Wari);

        assert_eq!(formatter.format("一割五分"), Some("0.15".to_string()));
    }
}
