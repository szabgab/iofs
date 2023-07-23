use std::fmt::Display;


pub trait And {
    type Output;
    fn and<T: Display>(self, other: T) -> Self::Output;
    fn with<T: Display>(&self, other: T) -> Self::Output;
    fn append<T: Display>(self, other: Vec<T>) -> Self::Output;
}


impl<S: Display> And for S  {
    type Output = String;

    fn and<T: Display>(self, other: T) -> Self::Output {
        format!("{}{}", self, other)
    }
    fn with<T: Display>(&self, other: T) -> Self::Output {
        format!("{}{}", self, other)
    }

    fn append<T: Display>(self, other: Vec<T>) -> Self::Output {
        let mut tmp = self.to_string();
        for str in other {
            tmp.push_str(str.to_string().as_str())
        }
        tmp
    }
}
