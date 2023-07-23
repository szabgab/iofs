use super::symbols::Symbols;

pub trait CharCompare {
    fn eq_with(&self, other: &Self) -> bool;
    fn eq_ingore_case(&self, other: &Self) -> bool;
}
impl CharCompare for char {
    fn eq_with(&self, other: &Self) -> bool {
        self == other           
    }

    fn eq_ingore_case(&self, other: &Self) -> bool {
        if self.is_letter() {
            self.to_lower() == other.to_lower()
        } else {
            self == other
        }
    }
}