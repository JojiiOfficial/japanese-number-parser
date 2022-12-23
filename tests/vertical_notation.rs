#[cfg(test)]
mod vertical_notation {
    use japanese_number_parser::JapaneseNumberFormatter;

    #[test]
    fn test_positional_notation() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(
            formatter.format("一二三、〇四五、〇六七・〇八九"),
            Some("123045067.089".to_string())
        );
        assert_eq!(
            formatter.format("一億二三〇四万五〇六七・〇八九"),
            Some("123045067.089".to_string())
        );
    }
}
