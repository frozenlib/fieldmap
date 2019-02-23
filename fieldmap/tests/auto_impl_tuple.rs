use fieldmap::*;

#[derive(FieldMap)]
#[field_map(item = "core::any::Any")]
struct ExampleType(u8, u16);

#[test]
fn test_get_by_idx() {
    use fieldmap::*;
    use std::any::Any;

    let value = ExampleType(10, 15);

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

    let value = ExampleType(10, 15);

    let mut iter = (&value).into_iter();

    assert_eq!(Any::downcast_ref::<u8>(iter.next().unwrap()), Some(&10));
    assert_eq!(Any::downcast_ref::<u16>(iter.next().unwrap()), Some(&15));
    assert!(iter.next().is_none());
}

#[test]
fn test_get_static() {
    use fieldmap::*;

    let value = ExampleType(10, 15);

    assert_eq!(FieldMapEntry::<u8>::get(&value), &10u8);
    assert_eq!(FieldMapEntry::<u16>::get(&value), &15u16);
}
