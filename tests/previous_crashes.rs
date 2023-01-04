#[cfg(test)]
mod previous_crashes {
    use japanese_number_parser::JapaneseNumberFormatter;

    #[test]
    fn crashes() {
        let formatter = JapaneseNumberFormatter::new();

        assert_eq!(formatter.format("Д"), None, "Test");
        // assert_eq!(formatter.format(" 億0"), None, "Test");
    }
}
