use similar::{ChangeTag, TextDiff};

pub fn diff<'a, 'b, 'c>(old: &'a str, new: &'b str) -> TextDiff<'a, 'b, 'c, str> {
    TextDiff::from_lines(old, new)
}
