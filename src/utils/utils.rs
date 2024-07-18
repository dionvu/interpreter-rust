pub fn is_letter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

pub fn is_digit(c: char) -> bool {
  c.is_digit(10)
}
