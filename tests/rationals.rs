#[cfg(test)]
mod rationals {
    use japanese_number_parser::JapaneseNumberFormatter;

    #[test]
    fn test_positional_notation() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("123456789.123"),
            Some("123456789.123".to_string())
        );
        assert_eq!(
            formatter.format("1234万5678.123"),
            Some("12345678.123".to_string())
        );
    }

    #[test]
    fn test_spelled_out() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("百二十三点〇一二"),
            Some("123.012".to_string())
        );
        assert_eq!(
            formatter.format("百二十三・〇一二"),
            Some("123.012".to_string())
        );
        assert_eq!(
            formatter.format("百二十三・零一二"),
            Some("123.012".to_string())
        );
    }

    #[test]
    fn decimal_point_without_rational() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(formatter.format("123点"), None);
        assert_eq!(formatter.format("123・"), None);
    }

    #[test]
    fn decimal_point_without_integer() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(formatter.format("点123"), None);
        assert_eq!(formatter.format("・123"), None);
    }
}
