# fieldmap

[![Crates.io](https://img.shields.io/crates/v/fieldmap.svg)](https://crates.io/crates/fieldmap)
[![Docs.rs](https://docs.rs/fieldmap/badge.svg)](https://docs.rs/crate/fieldmap)
[![Build Status](https://travis-ci.org/frozenlib/fieldmap.svg?branch=master)](https://travis-ci.org/frozenlib/fieldmap)

Zero cost compile-time map based on struct.

## Derive `Field`

`#[derive(Field)]` implements `Field`.

Following example implement `Field<u8>`, `Field<u16>`, `Field<String>` and access field by field type.

```rust
use fieldmap::Field;

#[derive(Field)]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
    value_s: String,
}

let x = ExampleType {
    value_u8: 100,
    value_u16: 200,
    value_s: "300".into(),
};

assert_eq!(*Field::<u8>::get(&x), 100);
assert_eq!(*Field::<u16>::get(&x), 200);
assert_eq!(*Field::<String>::get(&x), "300");
```

`#[derive(Field)]` can use only struct with different type of each field.

## Derive `Fields`

`#[derive(Fields)]` implements [`Fields`].

You need to specify `#[fields(item = "{TraitName}")]`.

```rust
use fieldmap::Fields;
use std::fmt::Debug;

#[derive(Fields)]
#[fields(item = "Debug")]
struct ExampleType {
    value_u8: u8,
    value_u16: u16,
    value_s: String,
}

let x = ExampleType {
    value_u8: 100,
    value_u16: 200,
    value_s: "300".into(),
};

for a in x.iter() {
    println!("{:?}", a);
}
```
Output:
```text
100
200
"300"
```

## Limitation
Only `'static` type can implement `Fields`.
Because this limitation is caused by Rust not supporting GAT (generic associated types),
so the limitation may be removed in the future.


## License
This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-* files for details.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
