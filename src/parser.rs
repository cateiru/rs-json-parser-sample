use crate::body::Body;
use crate::symbol;
use crate::value;

pub fn start(body: &mut Body) -> Option<Box<dyn value::Node>> {
    let target = body.pop()?;

    if symbol::is_start_brace(target) {
        // {
        start_brace(body)
    } else if symbol::is_start_bracket(target) {
        // [
        start_bracket(body)
    } else if symbol::is_blank(target) {
        // sparse
        start(body)
    } else if symbol::is_next(target) {
        // newline
        start(body)
    } else {
        None
    }
}

fn start_brace(body: &mut Body) -> Option<Box<dyn value::Node>> {
    None
}

fn start_bracket(body: &mut Body) -> Option<Box<dyn value::Node>> {
    None
}
