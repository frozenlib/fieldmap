pub use fieldmap_derive::FieldMap;

pub trait FieldMap: Sized + 'static {
    // TODO : use generic associated type. (`type Item<'a> : ?Sized;`)
    type Item: ?Sized;

    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> Option<&Self::Item>;
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item>;

    fn iter(&self) -> Iter<Self> {
        Iter { m: self, idx: 0 }
    }
    fn iter_mut(&mut self) -> IterMut<Self> {
        IterMut { m: self, idx: 0 }
    }
}

pub trait Field<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
    fn replace(&mut self, value: T) -> T;
}

pub struct Iter<'a, M> {
    m: &'a M,
    idx: usize,
}

impl<'a, M: FieldMap> Iterator for Iter<'a, M> {
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
impl<'a, M: FieldMap> ExactSizeIterator for Iter<'a, M> {}

pub struct IterMut<'a, M> {
    m: &'a mut M,
    idx: usize,
}

impl<'a, M: FieldMap> Iterator for IterMut<'a, M> {
    type Item = &'a mut M::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.m.get_mut(self.idx);
        if value.is_some() {
            self.idx += 1;
        }
        unsafe { ::core::mem::transmute(value) }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.m.len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, M: FieldMap> ExactSizeIterator for IterMut<'a, M> {}
