use fieldmap::Field;
use std::fmt::Debug;

#[derive(Field)]
#[field_map(item = "Debug")]
struct TupleType(u8, u16);

#[derive(Field)]
#[field_map(item = "Debug")]
struct GenericType<T: Debug + 'static> {
    v1: (T, T),
    v2: u16,
}
