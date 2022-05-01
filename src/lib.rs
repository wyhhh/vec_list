#![allow(clippy::collapsible_else_if)]
use core::fmt;
use std::hint::unreachable_unchecked;
use std::ops;
use std::ptr;

/// Double Linked List Backed by Vec
#[derive(Debug, Default)]
pub struct VecList<T> {
    list: Vec<Slot<T>>,
    head: Option<usize>,
    tail: Option<usize>,
    deleted_tail: Option<usize>,
    len: usize,
}

#[derive(Debug)]
enum Slot<T> {
    Value {
        val: T,
        next: Option<usize>,
        prev: Option<usize>,
    },
    Deleted {
        prev: Option<usize>,
    },
}

impl<T> Slot<T> {
    fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted { .. })
    }

    fn has_value(&self) -> bool {
        matches!(self, Self::Value { .. })
    }
}

impl<T> VecList<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            len: 0,
            head: None,
            tail: None,
            deleted_tail: None,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            list: Vec::with_capacity(cap),
            len: 0,
            head: None,
            tail: None,
            deleted_tail: None,
        }
    }

    /// Average O(1)
    pub fn push_back(&mut self, val: T) -> usize {
        let ret = if let Some(deleted_idx) = self.deleted_tail {
            let old_tail = self.tail;
            let deleted_slot = unsafe { self.get_slot_mut(deleted_idx) };

            debug_assert!(deleted_slot.is_deleted());
            let deleted_prev = match deleted_slot {
                Slot::Deleted { prev } => *prev,
                _ => unsafe { unreachable_unchecked() },
            };

            *deleted_slot = Slot::Value {
                val,
                next: None,
                prev: old_tail,
            };

            /* link old tail's next to new element */
            if let Some(old_tail) = self.tail {
                let old_tail = unsafe { self.get_slot_mut(old_tail) };

                debug_assert!(old_tail.has_value());

                match old_tail {
                    Slot::Value { next, .. } => *next = Some(deleted_idx),
                    _ => unsafe { unreachable_unchecked() },
                }
            }

            self.deleted_tail = deleted_prev;
            self.tail = Some(deleted_idx);

            if self.is_empty() {
                self.head = self.tail;
            }

            deleted_idx
        } else {
            let cur_idx = self.len();

            self.list.push(Slot::Value {
                val,
                next: None,
                prev: self.tail,
            });

            if self.is_empty() {
                self.head = Some(0);
            } else {
                debug_assert!(self.tail.is_some());
                let old_tail =
                    unsafe { self.get_slot_mut(unsafe { self.tail.unwrap_unchecked() }) };

                debug_assert!(old_tail.has_value());
                match old_tail {
                    Slot::Value { next, .. } => *next = Some(cur_idx),
                    _ => unsafe { unreachable_unchecked() },
                }
            }

            self.tail = Some(cur_idx);
            cur_idx
        };

        self.len += 1;
        ret
    }

    /// Average O(1)
    pub fn push_front(&mut self, val: T) -> usize {
        let ret = if let Some(deleted_idx) = self.deleted_tail {
            let old_head = self.head;
            let deleted_slot = unsafe { self.get_slot_mut(deleted_idx) };

            debug_assert!(deleted_slot.is_deleted());

            let deleted_prev = match deleted_slot {
                Slot::Deleted { prev } => *prev,
                _ => unsafe { unreachable_unchecked() },
            };

            *deleted_slot = Slot::Value {
                val,
                next: old_head,
                prev: None,
            };

            /* link old head's next to new element */
            if let Some(old_head) = self.head {
                let old_tail = unsafe { self.get_slot_mut(old_head) };

                debug_assert!(old_tail.has_value());
                match old_tail {
                    Slot::Value { prev, .. } => *prev = Some(deleted_idx),
                    _ => unsafe { unreachable_unchecked() },
                }
            }

            self.deleted_tail = deleted_prev;
            self.head = Some(deleted_idx);

            if self.is_empty() {
                self.tail = self.head;
            }

            deleted_idx
        } else {
            let cur_idx = self.len();

            self.list.push(Slot::Value {
                val,
                next: self.head,
                prev: None,
            });

            if self.is_empty() {
                self.tail = Some(0);
            } else {
                debug_assert!(self.head.is_some());
                let old_head =
                    unsafe { self.get_slot_mut(unsafe { self.head.unwrap_unchecked() }) };
                debug_assert!(old_head.has_value());

                match old_head {
                    Slot::Value { prev, .. } => *prev = Some(cur_idx),
                    _ => unsafe { unreachable_unchecked() },
                }
            }

            self.head = Some(cur_idx);
            cur_idx
        };

        self.len += 1;
        ret
    }

    /// O(1)
    pub fn pop_front(&mut self) -> Option<T> {
        self.delete(self.head?)
    }

    /// O(1)
    pub fn pop_back(&mut self) -> Option<T> {
        self.delete(self.tail?)
    }

    /// O(1)
    pub fn front(&self) -> Option<&T> {
        unsafe {
            debug_assert!(self.get_slot(self.head?).has_value());
            match self.get_slot(self.head?) {
                Slot::Value { val, .. } => Some(val),
                _ => unreachable_unchecked(),
            }
        }
    }

    /// O(1)
    pub fn back(&self) -> Option<&T> {
        unsafe {
            debug_assert!(self.get_slot(self.tail?).has_value());
            match self.get_slot(self.tail?) {
                Slot::Value { val, .. } => Some(val),
                _ => unreachable_unchecked(),
            }
        }
    }

    /// O(1)
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            debug_assert!(self.get_slot(self.head?).has_value());
            match self.get_slot_mut(self.head?) {
                Slot::Value { val, .. } => Some(val),
                _ => unreachable_unchecked(),
            }
        }
    }

    /// O(1)
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            debug_assert!(self.get_slot(self.tail?).has_value());
            match self.get_slot_mut(self.tail?) {
                Slot::Value { val, .. } => Some(val),
                _ => unreachable_unchecked(),
            }
        }
    }

    /// O(1)
    pub fn delete(&mut self, idx: usize) -> Option<T> {
        assert!(idx < self.cap());

        let old_delete_head = self.deleted_tail;
        let to_delete = unsafe { self.get_slot_mut(idx) } as *mut Slot<T>;

        /* connect links */
        let deleted_val = match unsafe { &mut *to_delete } {
            Slot::Value { next, prev, val } => {
                let to_delete_next = *next;
                let to_delete_prev = *prev;

                debug_assert!(self.head.is_some());
                debug_assert!(self.tail.is_some());
                /* solve head && tail */
                if unsafe { self.head.unwrap_unchecked() } == idx {
                    self.head = to_delete_next;
                }
                if unsafe { self.tail.unwrap_unchecked() } == idx {
                    self.tail = to_delete_prev;
                }

                /* solve previous */
                if let Some(prev) = to_delete_prev {
                    let prev = unsafe { self.get_slot_mut(prev) };

                    debug_assert!(prev.has_value());
                    match prev {
                        Slot::Value { next, .. } => {
                            *next = to_delete_next;
                        }
                        Slot::Deleted { .. } => unsafe { unreachable_unchecked() },
                    }
                }

                /* solve next */
                if let Some(next) = to_delete_next {
                    let next = unsafe { self.get_slot_mut(next) };

                    debug_assert!(next.has_value());
                    match next {
                        Slot::Value { prev, .. } => {
                            *prev = to_delete_prev;
                        }
                        Slot::Deleted { .. } => unsafe { unreachable_unchecked() },
                    }
                }

                unsafe { ptr::read(val) }
            }
            Slot::Deleted { .. } => return None,
        };

        /* set to empty */
        unsafe {
            ptr::write(
                to_delete,
                Slot::Deleted {
                    prev: old_delete_head,
                },
            );
        }

        self.deleted_tail = Some(idx);
        self.len -= 1;

        Some(deleted_val)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.list.len()
    }

    pub fn vec_cap(&self) -> usize {
        self.list.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            list: self,
            prev: self.tail,
            next: self.head,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let next = self.head;
        let prev = self.tail;

        IterMut {
            list: self,
            next,
            prev,
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        match self.list.get(idx) {
            Some(Slot::Value { val, .. }) => Some(val),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        match self.list.get_mut(idx) {
            Some(Slot::Value { val, .. }) => Some(val),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.len = 0;
        self.head = None;
        self.tail = None;
        self.deleted_tail = None;
    }

    // SAFETY: Must in range
    unsafe fn get_slot_mut(&mut self, idx: usize) -> &mut Slot<T> {
        debug_assert!(idx < self.cap());

        self.list.get_unchecked_mut(idx)
    }

    // SAFETY: Must in range
    unsafe fn get_slot(&self, idx: usize) -> &Slot<T> {
        debug_assert!(idx < self.cap());

        self.list.get_unchecked(idx)
    }
}

pub struct Iter<'a, T> {
    list: &'a VecList<T>,
    next: Option<usize>,
    prev: Option<usize>,
}

pub struct IterMut<'a, T> {
    list: &'a mut VecList<T>,
    next: Option<usize>,
    prev: Option<usize>,
}

pub struct IntoIter<T> {
    list: VecList<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next?;

        let slot = unsafe { self.list.get_slot(next) };

        debug_assert!(slot.has_value());

        match slot {
            Slot::Value { val, next, .. } => {
                self.next = *next;
                Some(val)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev = self.prev?;

        let slot = unsafe { self.list.get_slot(prev) };

        debug_assert!(slot.has_value());

        match slot {
            Slot::Value { val, prev, .. } => {
                self.prev = *prev;
                Some(val)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next?;

        let slot = unsafe { &mut *(self.list.get_slot_mut(next) as *mut Slot<T>) };

        debug_assert!(slot.has_value());

        match slot {
            Slot::Value { val, next, .. } => {
                self.next = *next;
                Some(val)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev = self.prev?;

        let slot = unsafe { &mut *(self.list.get_slot_mut(prev) as *mut Slot<T>) };

        debug_assert!(slot.has_value());

        match slot {
            Slot::Value { val, prev, .. } => {
                self.prev = *prev;
                Some(val)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ops::Index<usize> for VecList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("invalid key!")
    }
}

impl<T> ops::IndexMut<usize> for VecList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("invalid key!")
    }
}

impl<T> IntoIterator for VecList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a VecList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut VecList<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: fmt::Debug> fmt::Display for VecList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "[]")?;
            return Ok(());
        }

        let mut sep = "";
        for elt in self.iter() {
            write!(f, "{}{:?}", sep, elt)?;
            sep = " -> ";
        }
        writeln!(f)?;

        Ok(())
    }
}

#[test]
fn test() {
    let mut l = VecList::new();
    let a = l.push_back(1);
    let b = l.push_back(2);
    let c = l.push_back(3);
    let a = l.push_front(1);
    let b = l.push_front(2);
    let c = l.push_front(3);
    // let x = l.pop_back();
    // println!("{:?}",x);
    // let x = l.pop_back();
    // println!("{:?}",x);
    // let x = l.pop_back();
    // println!("{:?}",x);
    // let x = l.pop_back();
    // println!("{:?}",x);
    // let x = l.pop_front();
    // println!("{:?}",x);
    // let x = l.pop_front();
    // println!("{:?}",x);
    // let x = l.pop_front();
    // println!("{:?}",x);
    // let x = l.pop_front();
    // println!("{:?}",x);
    l.delete(a);
    l.delete(b);
    let c = l.push_back(4);
    let d = l.push_back(5);
    let e = l.push_back(6);
    l.delete(e);
    l.delete(d);

    for x in l.iter().rev() {
        println!("{:?}", x);
    }
    println!("{:?}", l.front());
    println!("{:?}", l.back());
    println!("{:#?}", l);
    println!("{}", l);
}
