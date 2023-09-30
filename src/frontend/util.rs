#[inline]
pub(crate) fn is_special_symbol(i: char) -> bool {
    matches!(i, '+' | '-' | '*' | '/' | '=' | '<' | '>' | ';')
}