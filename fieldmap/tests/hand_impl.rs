struct ExampleType {
    value_u8: u8,
    value_u16: u16,
}

impl ::fieldmap::FieldMap for ExampleType {
    type Item = dyn std::any::Any;

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

impl ExampleType {
    pub fn iter(&self) -> ::fieldmap::FieldMapIter<Self> {
        ::fieldmap::FieldMapIter::new(self)
    }
    pub fn iter_mut(&mut self) -> ::fieldmap::FieldMapIterMut<Self> {
        ::fieldmap::FieldMapIterMut::new(self)
    }
}

impl<'a> ::core::iter::IntoIterator for &'a ExampleType {
    type Item = &'a dyn std::any::Any;
    type IntoIter = ::fieldmap::FieldMapIter<'a, ExampleType>;

    fn into_iter(self) -> Self::IntoIter {
        ::fieldmap::FieldMapIter::new(self)
    }
}
impl<'a> ::core::iter::IntoIterator for &'a mut ExampleType {
    type Item = &'a mut dyn std::any::Any;
    type IntoIter = ::fieldmap::FieldMapIterMut<'a, ExampleType>;

    fn into_iter(self) -> Self::IntoIter {
        ::fieldmap::FieldMapIterMut::new(self)
    }
}

impl ::fieldmap::FieldMapEntry<u8> for ExampleType {
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

impl ::fieldmap::FieldMapEntry<u16> for ExampleType {
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

#[test]
fn test_get_by_idx() {
    use fieldmap::*;
    use std::any::Any;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };
    assert_eq!(
        Any::downcast_ref::<u8>(FieldMap::get(&value, 0).unwrap()),
        Some(&10)
    );
    assert_eq!(
        Any::downcast_ref::<u16>(FieldMap::get(&value, 1).unwrap()),
        Some(&15)
    );
}

#[test]
fn test_iter() {
    use std::any::Any;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    let mut iter = (&value).iter();

    assert_eq!(Any::downcast_ref::<u8>(iter.next().unwrap()), Some(&10));
    assert_eq!(Any::downcast_ref::<u16>(iter.next().unwrap()), Some(&15));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_mut() {
    use std::any::Any;

    let mut value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    {
        let mut iter = value.iter_mut();

        let r0 = Any::downcast_mut::<u8>(iter.next().unwrap()).unwrap();
        assert_eq!(*r0, 10);
        *r0 = 100;

        let r1 = Any::downcast_mut::<u16>(iter.next().unwrap()).unwrap();
        assert_eq!(*r1, 15);
        *r1 = 200;

        assert!(iter.next().is_none());
    }
    assert_eq!(value.value_u8, 100);
    assert_eq!(value.value_u16, 200);
}

#[test]
fn test_get_static() {
    use fieldmap::*;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    assert_eq!(FieldMapEntry::<u8>::get(&value), &10u8);
    assert_eq!(FieldMapEntry::<u16>::get(&value), &15u16);
}
