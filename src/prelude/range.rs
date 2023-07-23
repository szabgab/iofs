pub trait Range<T> {
    fn range_at(&self, s: &T, e: &T) -> bool;
    fn contained_in(&self, pats: &[T]) -> bool;
}

impl<T: PartialEq + PartialOrd> Range<T> for T {
    fn range_at(&self, s: &T, e: &T) -> bool {
        self >= s && e >= self
    }

    fn contained_in(&self, pats: &[T]) -> bool {
        for item in pats {
            if item == self { return true }
        }
        false
    }
}