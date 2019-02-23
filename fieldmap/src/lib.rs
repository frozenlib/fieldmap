pub use fieldmap_derive::FieldMap;

pub trait FieldMap {
    type Item: ?Sized;

    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> Option<&Self::Item>;
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item>;

    fn for_each(&self, mut f: impl FnMut(&Self::Item)) {
        for idx in 0..self.len() {
            if let Some(value) = self.get(idx) {
                f(value);
            }
        }
    }
    fn for_each_mut(&mut self, mut f: impl FnMut(&mut Self::Item)) {
        for idx in 0..self.len() {
            if let Some(value) = self.get_mut(idx) {
                f(value);
            }
        }
    }
}

pub trait FieldMapEntry<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
    fn replace(&mut self, value: T) -> T;
}

pub struct FieldMapIter<'a, M> {
    m: &'a M,
    idx: usize,
}
impl<'a, M> FieldMapIter<'a, M> {
    pub fn new(m: &'a M) -> Self {
        FieldMapIter { m, idx: 0 }
    }
}

impl<'a, M: FieldMap> Iterator for FieldMapIter<'a, M> {
    type Item = &'a M::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.m.get(self.idx);
        if value.is_some() {
            self.idx += 1;
        }
        value
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.m.len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, M: FieldMap> ExactSizeIterator for FieldMapIter<'a, M> {}

pub struct FieldMapIterMut<'a, M> {
    m: &'a mut M,
    idx: usize,
}
impl<'a, M> FieldMapIterMut<'a, M> {
    pub fn new(m: &'a mut M) -> Self {
        FieldMapIterMut { m, idx: 0 }
    }
}

impl<'a, M: FieldMap> Iterator for FieldMapIterMut<'a, M> {
    type Item = &'a mut M::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.m.get_mut(self.idx);
        if let Some(value) = value {
            self.idx += 1;
            unsafe { Some(::core::mem::transmute(value)) }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.m.len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, M: FieldMap> ExactSizeIterator for FieldMapIterMut<'a, M> {}
