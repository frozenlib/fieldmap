use fieldmap::{Field, Fields};
use std::fmt::Debug;

#[derive(Field, Fields)]
#[fields(item = "Debug")]
struct TupleType(u8, u16);

#[derive(Field, Fields)]
#[fields(item = "Debug")]
struct GenericType<T: Debug + 'static> {
    v1: (T, T),
    v2: u16,
}
