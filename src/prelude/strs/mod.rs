use super::compare::CharCompare;

pub trait Str {
    fn match_with(&self, other: &Self) -> Option<Vec<usize>>;
}

impl Str for &str {
    fn match_with(&self, other: &Self) -> Option<Vec<usize>> {
        let mut pos = Vec::new();
        let mut my_index = 0;
        let mut other_index = 0;
        let my_chars = self.chars().collect::<Vec<_>>();
        let other_chars = other.chars().collect::<Vec<_>>();
        while my_index < my_chars.len() && other_index < other_chars.len() {
            if my_chars[my_index].eq_ingore_case(&other_chars[other_index]) {
                pos.push(other_index);
                my_index += 1;
                other_index += 1;
            } else {
                my_index += 1;
            }
        }
        if pos.is_empty() {
            None
        } else {
            Some(pos)
        }
    }
}
