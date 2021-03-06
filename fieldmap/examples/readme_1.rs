fn main() {
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

    for (name, value) in x.iter() {
        println!("{} = {:?}", name, value);
    }
}
