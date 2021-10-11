/// Check if the character is `{`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_start_brace(target: &u8) -> bool {
    // `{` is 123
    comparison_symbol(target, &123u8)
}

/// Check if the character is `}`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_end_brace(target: &u8) -> bool {
    // `}` is 125
    comparison_symbol(target, &125u8)
}

/// Check if the character is `[`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_start_bracket(target: &u8) -> bool {
    // `[` is 91
    comparison_symbol(target, &91u8)
}

/// Check if the character is `}`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_end_bracket(target: &u8) -> bool {
    // `]` is 93
    comparison_symbol(target, &93u8)
}

/// Check if the character is `,`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_comma(target: &u8) -> bool {
    // `,` is 44
    comparison_symbol(target, &44u8)
}

/// Check if the character is `:`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_colon(target: &u8) -> bool {
    // `:` is 58
    comparison_symbol(target, &58u8)
}

/// Check if the character is `"`.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_double_quotation(target: &u8) -> bool {
    // `"` is 34
    comparison_symbol(target, &34u8)
}

/// Check if the character is whitespace.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_blank(target: &u8) -> bool {
    // whitespace is 32
    comparison_symbol(target, &32u8)
}

/// Check if the character is newline.
///
/// ## Arguments
///
/// - `target` - target string.
pub fn is_next(target: &u8) -> bool {
    // newline(\n) is 10
    comparison_symbol(target, &10u8)
}

/// Compares two types of UTF-8 characters.
/// True if they are the same. If not, it returns false.
///
/// ## Arguments
///
/// - `target` - target string.
/// - `symbol` - string to be compared.
fn comparison_symbol(target: &u8, symbol: &u8) -> bool {
    if target == symbol {
        true
    } else {
        false
    }
}
