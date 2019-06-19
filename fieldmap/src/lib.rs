/*!
Zero cost compile-time map based on struct.

## Derive `Field`

`#[derive(Field)]` implements [`Field`].

Following example implement `Field<u8>`, `Field<u16>`, `Field<String>` and access field by field type.

```rust
use fieldmap::Field;

#[derive(Field)]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
    value_s: String,
}

let x = ExampleType {
    value_u8: 100,
    value_u16: 200,
    value_s: "300".into(),
};

assert_eq!(*Field::<u8>::get(&x), 100);
assert_eq!(*Field::<u16>::get(&x), 200);
assert_eq!(*Field::<String>::get(&x), "300");
```

`#[derive(Field)]` can use only struct with different type of each field.

## Derive `Fields`

`#[derive(Fields)]` implements [`Fields`].

You need to specify `#[fields(item = "{TraitName}")]`.

```rust
use fieldmap::Fields;
use std::fmt::Debug;

#[derive(Fields)]
#[fields(item = "Debug")]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
    value_s: String,
}

let x = ExampleType {
    value_u8: 100,
    value_u16: 200,
    value_s: "300".into(),
};

for (name, value) in x.iter() {
    println!("{} = {:?}", name, value);
}
```
Output:
```text
value_u8 = 100
value_u16 = 200
value_s = "300"
```

## Limitation
Only `'static` type can implement [`Fields`].
Because this limitation is caused by Rust not supporting GAT (generic associated types),
so the limitation may be removed in the future.
*/

use std::iter::FusedIterator;
use std::marker::PhantomData;

pub use fieldmap_derive::{Field, Fields};

/// An interface for access all fields.
///
/// See the [module-level documentation](index.html) for more details.
pub trait Fields: Sized + 'static {
    // TODO : use generic associated type. (`type Item<'a> : ?Sized;`)
    type Item: ?Sized;

    fn len() -> usize;
    fn find(name: &str) -> Option<usize>;
    fn name(idx: usize) -> Option<&'static str>;

    fn get(&self, idx: usize) -> Option<&Self::Item>;
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item>;

    fn iter(&self) -> Iter<Self> {
        Iter { s: self, idx: 0 }
    }
    fn iter_mut(&mut self) -> IterMut<Self> {
        IterMut { s: self, idx: 0 }
    }
    fn values(&self) -> Values<Self> {
        Values { s: self, idx: 0 }
    }
    fn values_mut(&mut self) -> ValuesMut<Self> {
        ValuesMut { s: self, idx: 0 }
    }
    fn names() -> Names<Self> {
        Names {
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

/// An interface for access one field by field type.
///
/// See the [module-level documentation](index.html) for more details.
pub trait Field<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

/// Immutable field iterator of [`Fields`].
pub struct Iter<'a, S> {
    s: &'a S,
    idx: usize,
}

impl<'a, S: Fields> Iterator for Iter<'a, S> {
    type Item = (&'static str, &'a S::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(name), Some(value)) = (S::name(self.idx), self.s.get(self.idx)) {
            self.idx += 1;
            Some((name, value))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = S::len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, S: Fields> ExactSizeIterator for Iter<'a, S> {}
impl<'a, S: Fields> FusedIterator for Iter<'a, S> {}

/// Mmutable field iterator of [`Fields`].
pub struct IterMut<'a, S> {
    s: &'a mut S,
    idx: usize,
}

impl<'a, S: Fields> Iterator for IterMut<'a, S> {
    type Item = (&'static str, &'a mut S::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(name), Some(value)) = (S::name(self.idx), self.s.get_mut(self.idx)) {
            self.idx += 1;
            Some((name, unsafe { ::core::mem::transmute(value) }))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = S::len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, S: Fields> ExactSizeIterator for IterMut<'a, S> {}
impl<'a, S: Fields> FusedIterator for IterMut<'a, S> {}

pub struct Values<'a, S> {
    s: &'a S,
    idx: usize,
}

impl<'a, M: Fields> Iterator for Values<'a, M> {
    type Item = &'a M::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.s.get(self.idx) {
            self.idx += 1;
            Some(value)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = M::len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, M: Fields> ExactSizeIterator for Values<'a, M> {}
impl<'a, M: Fields> FusedIterator for Values<'a, M> {}

pub struct ValuesMut<'a, S> {
    s: &'a mut S,
    idx: usize,
}

impl<'a, S: Fields> Iterator for ValuesMut<'a, S> {
    type Item = &'a mut S::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.s.get_mut(self.idx) {
            self.idx += 1;
            Some(unsafe { ::core::mem::transmute(value) })
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = S::len() - self.idx;
        (size, Some(size))
    }
}
impl<'a, S: Fields> ExactSizeIterator for ValuesMut<'a, S> {}
impl<'a, S: Fields> FusedIterator for ValuesMut<'a, S> {}

pub struct Names<S> {
    idx: usize,
    _phantom: PhantomData<fn(&S)>,
}

impl<S: Fields> Iterator for Names<S> {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = S::name(self.idx) {
            self.idx += 1;
            Some(value)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = S::len() - self.idx;
        (size, Some(size))
    }
}
impl<S: Fields> ExactSizeIterator for Names<S> {}
impl<S: Fields> FusedIterator for Names<S> {}
