pub fn loosely_parse_i32(string: &str) -> i32 {
    match string.parse::<i32>() {
        Ok(int) => int,
        Err(e) => {
            use std::num::IntErrorKind as IEK;
            match e.kind() {
                IEK::PosOverflow => i32::MAX,
                _ => i32::MIN,
            }
        }
    }
}
/// More slint aligned version of the inner function.
pub fn loosely_parse_int(string: slint::SharedString) -> i32 {
    loosely_parse_i32(&string)
}
