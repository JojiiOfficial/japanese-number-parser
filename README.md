# Japanese Number Parser

A minimal rust library for converting Japanese number strings into arabic number
strings. Handles all sorts of formats and styles, including very big numbers
(such as 10<sup>60</sup>, or 那由他), both style of fractional numbers, and so on.

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

# Different parsing styles

The formatter can be set up, so that it treats fractional numbers using the
"discount" system (so 0.1 is 割 instead of 分).

Similarly, very large numbers are sometimes understood to be 10<sup>8</sup>
based instead of 10<sup>4</sup>. This can also be set.

```rust
let mut formatter = JapaneseNumberFormatter::new();
formatter.very_large_number_handling(VeryLargeNumberHandling::Alternate);
formatter.fractional_handling(FractionalHandling::Wari);
```
