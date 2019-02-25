struct ExampleType {
    value_u8: u8,
    value_u16: u16,
}

// ==================
// Begin hand impl

impl ::fieldmap::Fields for ExampleType {
    type Item = dyn std::fmt::Display;

    #[inline]
    fn len(&self) -> usize {
        2
    }

    #[inline]
    fn get(&self, idx: usize) -> ::core::option::Option<&Self::Item> {
        match idx {
            0 => Some(&self.value_u8),
            1 => Some(&self.value_u16),
            _ => None,
        }
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> ::core::option::Option<&mut Self::Item> {
        match idx {
            0 => Some(&mut self.value_u8),
            1 => Some(&mut self.value_u16),
            _ => None,
        }
    }
}

impl<'_a> ::core::iter::IntoIterator for &'_a ExampleType {
    type Item = <::fieldmap::Iter<'_a, ExampleType> as Iterator>::Item;
    type IntoIter = ::fieldmap::Iter<'_a, ExampleType>;

    fn into_iter(self) -> Self::IntoIter {
        ::fieldmap::Fields::iter(self)
    }
}
impl<'_a> ::core::iter::IntoIterator for &'_a mut ExampleType {
    type Item = <::fieldmap::IterMut<'_a, ExampleType> as Iterator>::Item;
    type IntoIter = ::fieldmap::IterMut<'_a, ExampleType>;

    fn into_iter(self) -> Self::IntoIter {
        ::fieldmap::Fields::iter_mut(self)
    }
}

impl ::fieldmap::Field<u8> for ExampleType {
    #[inline]
    fn get(&self) -> &u8 {
        &self.value_u8
    }

    #[inline]
    fn get_mut(&mut self) -> &mut u8 {
        &mut self.value_u8
    }

    #[inline]
    fn replace(&mut self, value: u8) -> u8 {
        ::core::mem::replace(&mut self.value_u8, value)
    }
}

impl<'a> ::fieldmap::Field<u16> for ExampleType {
    #[inline]
    fn get(&self) -> &u16 {
        &self.value_u16
    }

    #[inline]
    fn get_mut(&mut self) -> &mut u16 {
        &mut self.value_u16
    }

    #[inline]
    fn replace(&mut self, value: u16) -> u16 {
        ::core::mem::replace(&mut self.value_u16, value)
    }
}

// End hand impl
// ==================

#[test]
fn test_get_by_idx() {
    use fieldmap::*;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };
    assert_eq!(format!("{}", Fields::get(&value, 0).unwrap()), "10");
    assert_eq!(format!("{}", Fields::get(&value, 1).unwrap()), "15");
    assert!(Fields::get(&value, 2).is_none());
}

#[test]
fn test_iter() {
    use fieldmap::*;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    let mut iter = value.iter();
    assert_eq!(format!("{}", iter.next().unwrap()), "10");
    assert_eq!(format!("{}", iter.next().unwrap()), "15");
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_mut() {
    use fieldmap::*;

    let mut value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    let mut iter = value.iter_mut();
    assert_eq!(format!("{}", iter.next().unwrap()), "10");
    assert_eq!(format!("{}", iter.next().unwrap()), "15");
    assert!(iter.next().is_none());
}

#[test]
fn test_get_static() {
    use fieldmap::*;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    assert_eq!(Field::<u8>::get(&value), &10u8);
    assert_eq!(Field::<u16>::get(&value), &15u16);
}
