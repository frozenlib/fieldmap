use fieldmap::*;

#[derive(FieldMap)]
#[field_map(item = "std::fmt::Debug")]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
}

#[test]
fn test_iter() {
    let value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    let mut iter = value.iter();

    assert_eq!(format!("{:?}", iter.next().unwrap()), "10");
    assert_eq!(format!("{:?}", iter.next().unwrap()), "15");

    assert!(iter.next().is_none());
}

#[test]
fn test_iter_mut() {
    let mut value = ExampleType {
        value_u8: 10,
        value_u16: 15,
    };

    let mut iter = value.iter_mut();

    assert_eq!(format!("{:?}", iter.next().unwrap()), "10");
    assert_eq!(format!("{:?}", iter.next().unwrap()), "15");

    assert!(iter.next().is_none());
}
