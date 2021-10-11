pub fn is_start_brace(target: &str) -> bool {
    comparison_symbol(target, "{")
}

pub fn is_end_brace(target: &str) -> bool {
    comparison_symbol(target, "}")
}

pub fn is_start_bracket(target: &str) -> bool {
    comparison_symbol(target, "[")
}

pub fn is_end_bracket(target: &str) -> bool {
    comparison_symbol(target, "]")
}

pub fn is_comma(target: &str) -> bool {
    comparison_symbol(target, ",")
}

pub fn is_colon(target: &str) -> bool {
    comparison_symbol(target, ":")
}

pub fn is_double_quotation(target: &str) -> bool {
    comparison_symbol(target, "\"")
}

pub fn is_blank(target: &str) -> bool {
    comparison_symbol(target, " ")
}

pub fn is_next(target: &str) -> bool {
    comparison_symbol(target, "\n")
}

fn comparison_symbol(target: &str, symbol: &str) -> bool {
    match target {
        symbol => true,
        _ => false,
    }
}
