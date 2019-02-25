//! Zero cost compile-time map based on struct.
//!
//! # Derive [`Field`]
//!
//! `#[derive(Field)]` implements [`Field`].
//!
//! Following example implement `Field<u8>`, `Field<u16>`, `Field<String>` and access field by field type.
//!
//! ```rust
//! use fieldmap::Field;
//!
//! #[derive(Field)]
//! struct ExampleType {
//!     value_u8: u8,
//!     value_u16: u16,
//!     value_s: String,
//! }
//!
//! let x = ExampleType {
//!     value_u8: 100,
//!     value_u16: 200,
//!     value_s: "300".into(),
//! };
//!
//! assert_eq!(*Field::<u8>::get(&x), 100);
//! assert_eq!(*Field::<u16>::get(&x), 200);
//! assert_eq!(*Field::<String>::get(&x), "300");
//! ```
//!
//! `#[derive(Field)]` can use only struct with different type of each field.
//!
//! # Derive [`Fields`]
//!
//! `#[derive(Fields)]` implements [`Fields`].
//!
//! You need to specify `#[fields(item = "{TraitName}")]`.
//!
//! ```rust
//! use fieldmap::Fields;
//! use std::fmt::Debug;
//!
//! #[derive(Fields)]
//! #[fields(item = "Debug")]
//! struct ExampleType {
//!     value_u8: u8,
//!     value_u16: u16,
//!     value_s: String,
//! }
//!
//! let x = ExampleType {
//!     value_u8: 100,
//!     value_u16: 200,
//!     value_s: "300".into(),
//! };
//!
//! for a in x.iter() {
//!     println!("{:?}", a);
//! }
//! ```
//! Output:
//! ```text
//! 100
//! 200
//! "300"
//! ```
//!
//! # Limitation
//!
//! Only `'static` type can implement [`Fields`].
//!
//! Because this limitation is caused by Rust not supporting GAT (generic associated types),
//! so the limitation may be removed in the future.
//!

use std::iter::FusedIterator;

pub use fieldmap_derive::{Field, Fields};

/// An interface for access all fields.
///
/// See the [module-level documentation](index.html) for more details.
pub trait Fields: Sized + 'static {
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

/// An interface for access one field by field type.
///
/// See the [module-level documentation](index.html) for more details.
pub trait Field<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
    fn replace(&mut self, value: T) -> T;
}

/// Immutable field iterator of [`Fields`].
pub struct Iter<'a, M> {
    m: &'a M,
    idx: usize,
}

impl<'a, M: Fields> Iterator for Iter<'a, M> {
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
impl<'a, M: Fields> ExactSizeIterator for Iter<'a, M> {}
impl<'a, M: Fields> FusedIterator for Iter<'a, M> {}

/// Mmutable field iterator of [`Fields`].
pub struct IterMut<'a, M> {
    m: &'a mut M,
    idx: usize,
}

impl<'a, M: Fields> Iterator for IterMut<'a, M> {
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
impl<'a, M: Fields> ExactSizeIterator for IterMut<'a, M> {}
impl<'a, M: Fields> FusedIterator for IterMut<'a, M> {}
