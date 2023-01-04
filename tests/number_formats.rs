#[cfg(test)]
mod number_formats {
    use japanese_number_parser::JapaneseNumberFormatter;

    #[test]
    fn test_positional_notation() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("123456789"),
            Some("123456789".to_string()),
            "Roman numerals"
        );
        assert_eq!(
            formatter.format("123,456,789"),
            Some("123456789".to_string()),
            "Roman numerals with separators"
        );
        assert_eq!(
            formatter.format("1234万5678"),
            Some("12345678".to_string()),
            "Roman numerals with japanese separators"
        );
        assert_eq!(
            formatter.format("1234億5678万"),
            Some("123456780000".to_string()),
            "Japanese separators with extra padding zeroes"
        );
        assert_eq!(
            formatter.format("１２３４５６７８９"),
            Some("123456789".to_string()),
            "Full-width numerals"
        );
        assert_eq!(
            formatter.format("１２３４万５６７８"),
            Some("12345678".to_string()),
            "Full-width numerals with japanese separators"
        );
        assert_eq!(
            formatter.format("１２３，４５６，７８９"),
            Some("123456789".to_string()),
            "Full-width numerals with separators"
        );
    }

    #[test]
    fn test_spelled_out() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("千二百三十四万五千六百七十八"),
            Some("12345678".to_string()),
            "Spelled out"
        );
        assert_eq!(
            formatter.format("1千8百十"),
            Some("1810".to_string()),
            "Mixed style"
        )
    }

    #[test]
    fn test_anti_forgery() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("壱億弐阡参百四拾五萬六阡七百八拾九"),
            Some("123456789".to_string()),
            "Spelled out with anti-forgery"
        );
    }

    #[test]
    fn test_financial() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("１百万３４５千６７８"),
            Some("1345678".to_string()),
            "Financial notation"
        )
    }
}
