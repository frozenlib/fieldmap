use fieldmap::*;

#[derive(FieldMap)]
#[field_map(item = "core::any::Any")]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
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

    let mut iter = value.iter();

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
