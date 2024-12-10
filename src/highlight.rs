use crate::search::Position;

pub fn highlight(string: &str, position: Position) -> String {
    let mut new_string = String::new();

    new_string.push_str(&string[..position.start]);
    new_string.push_str("\x1b[1;31m");
    new_string.push_str(&string[position.start..position.end]);
    new_string.push_str("\x1b[0m");
    new_string.push_str(&string[position.end..]);

    new_string
}
