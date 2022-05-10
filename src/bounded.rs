use crate::Iter;
use crate::VecList;

/// A bounded list, when cap is full, it will `pop_back` and `push_front`
#[derive(Debug, Default, Clone)]
pub struct BoundedList<T> {
    list: VecList<T>,
    cap: usize,
}

impl<T> BoundedList<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            list: VecList::with_capacity(cap),
        }
    }

    pub fn add(&mut self, val: T) {
        if self.list.len() == self.cap {
            self.list.pop_back();
            self.list.push_front(val);
        } else {
            self.list.push_back(val);
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<T> {
        self.list.iter()
    }
}
