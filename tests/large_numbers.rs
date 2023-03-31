#[cfg(test)]
mod large_numbers {
    use japanese_number_parser::{JapaneseNumberFormatter, VeryLargeNumberHandling};

    #[test]
    fn test_large_numbers_regular() {
        let formatter = JapaneseNumberFormatter::new();

        assert_power_of_ten(&formatter, 32, "一溝");
        assert_power_of_ten(&formatter, 52, "一恒河沙");
    }

    #[test]
    fn test_large_numbers_alternate() {
        let mut formatter = JapaneseNumberFormatter::new();
        formatter.very_large_number_handling(VeryLargeNumberHandling::Alternate);

        assert_power_of_ten(&formatter, 64, "一阿僧祇");
    }

    fn assert_power_of_ten(formatter: &JapaneseNumberFormatter, power: i32, str: &str) {
        let number_option = formatter.format(str);
        assert!(number_option.is_some());
        let number = number_option.unwrap();
        let mut number_iter = number.chars();
        assert!(number_iter.next() == Some('1'));
        assert!(number_iter.all(|c| c == '0'));
        assert!(number.len() == (power + 1) as usize);
    }

    #[test]
    fn counterless_power() {
        let formatter = JapaneseNumberFormatter::new();
        let number_option = formatter.format("億");
        assert!(number_option.is_none());
    }

    #[test]
    fn zero_as_multiplier() {
        let formatter = JapaneseNumberFormatter::new();
        let number_option = formatter.format("000億");
        assert!(number_option.is_none());
    }
}
