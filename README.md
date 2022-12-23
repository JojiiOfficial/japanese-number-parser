# Japanese Number Parser

A minimal rust library for converting Japanese number strings into arabic number
strings. Handles all sorts of formats and styles, including very big numbers
(such as 10^60, or 那由他), both style of fractional numbers, and so on.

The results will be given in string format, this way the library doesn't rely on
any big number implementations, and therefore isn't as useful, if you need the
actual value behind it.

# Example

```rust
let formatter = JapaneseNumberFormatter::new();
assert_eq!(
    formatter.format("千二百三十四万五千六百七十八"),
    Some("12345678".to_string())
);
```
