pub trait Find<T> {
    fn find(&self, x: &T) -> Option<usize>;
    fn find_all(&self, x: &T) -> Option<Vec<usize>>;
    fn find_last(&self, x: &T) -> Option<usize>;
    fn finds(&self, x: &[T]) -> Option<(usize, usize)>;
    fn find_all_size(&self, x: &T) -> usize;
    fn last_eq(&self, x: &T) -> bool;
}

impl<T: PartialEq> Find<T> for &[T] {
    fn find(&self, x: &T) -> Option<usize> {
        for (p, item) in self.iter().enumerate() {
            if item == x {
                return Some(p);
            }
        }
        None
    }

    fn find_all(&self, x: &T) -> Option<Vec<usize>> {
        let mut value = Vec::new();
        for (p, item) in self.iter().enumerate() {
            if item == x {
                value.push(p)
            }
        }
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }

    fn find_last(&self, x: &T) -> Option<usize> {
        let mut index = self.len() - 1;
        while index != 0 {
            if &self[index] == x {
                return Some(index)
            }
            index -= 1;
        }
        None
    }

    fn finds(&self, needle: &[T]) -> Option<(usize, usize)> {
        if self.is_empty() || self.is_empty() || self.len() < self.len(){
            None
        } else if self == &needle {
           Some((0, self.len() - 1))
        } else {
            let mut index = 0;
            while index < self.len() {
                let end = index + needle.len();
                if end < self.len() {
                    if &self[index..end] == needle {
                        return Some((index, end - 1))
                    }
                    index += 1;
                } else { break; }
            }
            None
        }
    }

    fn find_all_size(&self, x: &T) -> usize {
        let mut count = 0;
        for item in self.iter() {
            if item == x {
                count += 1;
            }
        }
        count
    }

    fn last_eq(&self, x: &T) -> bool {
        match self.last() {
            Some(k) => k == x,
            _ => false
        }
    }
}

impl<T: PartialEq> Find<T> for Vec<T> {
    fn find(&self, x: &T) -> Option<usize> {
        self.as_slice().find(x)
    }
    fn find_all(&self, x: &T) -> Option<Vec<usize>> {
        self.as_slice().find_all(x)
    }

    fn find_last(&self, x: &T) -> Option<usize> {
        self.as_slice().find_last(x)
    }

    fn finds(&self, x: &[T]) -> Option<(usize, usize)> {
        self.as_slice().finds(x)
    }

    fn find_all_size(&self, x: &T) -> usize {
        self.as_slice().find_all_size(x)
    }

    fn last_eq(&self, x: &T) -> bool {
        self.as_slice().last_eq(x)
    }
}
