use std::fmt::Display;

struct ExampleType {
    value_u8: u8,
    value_u16: u16,
}

// ==================
// Begin hand impl

impl ::fieldmap::Fields for ExampleType {
    type Item = dyn std::fmt::Display;

    #[inline]
    fn len() -> usize {
        2
    }

    #[inline]
    fn name(idx: usize) -> ::core::option::Option<&'static str> {
        match idx {
            0 => Some("value_u8"),
            1 => Some("value_u16"),
            _ => None,
        }
    }

    #[inline]
    fn find(name: &str) -> ::core::option::Option<usize> {
        match name {
            "value_u8" => Some(0),
            "value_u16" => Some(1),
            _ => None,
        }
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
    assert_next(&mut iter, "value_u8", "10");
    assert_next(&mut iter, "value_u16", "15");
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
    assert_next(&mut iter, "value_u8", "10");
    assert_next(&mut iter, "value_u16", "15");
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

fn assert_next(
    iter: &mut impl Iterator<Item = (&'static str, impl Display)>,
    name: &str,
    value: &str,
) {
    if let Some((a_name, a_value)) = iter.next() {
        assert_eq!(
            format!("{} = {}", a_name, a_value),
            format!("{} = {}", name, value)
        );
    } else {
        panic!("next() return None.");
    }
}
