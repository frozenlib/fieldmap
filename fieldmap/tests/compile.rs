use fieldmap::FieldMap;
use std::fmt::Debug;

#[derive(FieldMap)]
#[field_map(item = "Debug")]
struct TupleType(u8, u16);

#[derive(FieldMap)]
#[field_map(item = "Debug")]
struct GenericType<T: Debug + 'static> {
    v1: (T, T),
    v2: u16,
}
