# enum-from-str-rs
Allow derive FromStr for enums 

## Overview
This library adds the ```#[derive(FromStr)]``` attribute for enums, thus allowing parsing strings into enum variants. Even though there are other libraries that allows this, like enum_derive, it only allows you to parse from strings with the exact name of a variant. This library allows you to define a custom string for each enum variant.

## Basic usage
Add the following lines to your project's dependencies:
```ini
enum_from_str = "0.1.0"
enum_from_str_derive = "0.1.0"
```

Example
```rust
use enum_from_str::ParseEnumVariantError;
use enum_from_str_derive::FromStr;

#[derive(FromStr)]
enum SomeEnum {
    #[from_str="foo"]
    Foo,
    Bar, // equals to #[from_str="Bar"]
}

fn example() {
    "foo".parse::<SomeEnum>().unwrap();
    "Bar".parse::<SomeEnum>().unwrap();
}
```

## Known issues
Currently, proc-macro crates doesn't allow exporting anything other than the proc-macro function. That's why ParseEnumVariantError is in a different crate. When Rust allows it, it should be moved into a single crate.
