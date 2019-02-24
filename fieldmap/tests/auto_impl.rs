use fieldmap::FieldMap;

#[derive(FieldMap)]
#[field_map(item = "std::fmt::Display")]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
}

#[test]
fn test_get_by_idx() {
    use fieldmap::*;

    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };
    assert_eq!(format!("{}", FieldMap::get(&value, 0).unwrap()), "10");
    assert_eq!(format!("{}", FieldMap::get(&value, 1).unwrap()), "15");
    assert!(FieldMap::get(&value, 2).is_none());
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
