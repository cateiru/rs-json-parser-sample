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

/// Check if the number.
/// 0 to 9 and `-`
///
/// TODO: `-` must be at the beginning of the number.
pub fn is_number(target: &u8) -> bool {
    if (*target >= 48u8 && *target < 58u8) || *target == 45u8 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol;

    #[test]
    fn brace() {
        let braces = "{}".as_bytes();

        assert_eq!(true, symbol::is_start_brace(&braces[0]));
        assert_eq!(false, symbol::is_start_brace(&braces[1]));

        assert_eq!(false, symbol::is_end_brace(&braces[0]));
        assert_eq!(true, symbol::is_end_brace(&braces[1]));
    }

    #[test]
    fn bracket() {
        let brackets = "[]".as_bytes();

        assert_eq!(true, symbol::is_start_bracket(&brackets[0]));
        assert_eq!(false, symbol::is_start_bracket(&brackets[1]));

        assert_eq!(false, symbol::is_end_bracket(&brackets[0]));
        assert_eq!(true, symbol::is_end_bracket(&brackets[1]));
    }

    #[test]
    fn comma() {
        let comma = ",.".as_bytes();

        assert_eq!(true, symbol::is_comma(&comma[0]));
        assert_eq!(false, symbol::is_comma(&comma[1]));
    }

    #[test]
    fn colon() {
        let colon = ":;".as_bytes();

        assert_eq!(true, symbol::is_colon(&colon[0]));
        assert_eq!(false, symbol::is_colon(&colon[1]));
    }

    #[test]
    fn double_quotation() {
        let dq = "\"\'".as_bytes();

        assert_eq!(true, symbol::is_double_quotation(&dq[0]));
        assert_eq!(false, symbol::is_double_quotation(&dq[1]));
    }

    #[test]
    fn blank() {
        let whitespace = " a".as_bytes();

        assert_eq!(true, symbol::is_blank(&whitespace[0]));
        assert_eq!(false, symbol::is_blank(&whitespace[1]));
    }

    #[test]
    fn newline() {
        let newline = "\n\t".as_bytes();

        assert_eq!(true, symbol::is_next(&newline[0]));
        assert_eq!(false, symbol::is_next(&newline[1]));
    }

    #[test]
    fn numbers() {
        let numbers = "1234567890-".as_bytes();

        for element in numbers {
            assert_eq!(true, symbol::is_number(element));
        }
    }
}
