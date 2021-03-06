#[macro_use]
extern crate honggfuzz;

use bp7::Bundle;
use std::convert::TryFrom;

fn main() {
    // Here you can parse `std::env::args and
    // setup / initialize your project

    // You have full control over the loop but
    // you're supposed to call `fuzz` ad vitam aeternam
    loop {
        // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
        // to a closure-like block of code.
        // For performance reasons, it is recommended that you use the native type
        // `&[u8]` when possible.
        // Here, this slice will contain a "random" quantity of "random" data.
        fuzz!(|data: &[u8]| {
            let deserialized: std::result::Result<Bundle, _> = Bundle::try_from(Vec::from(data));
            if deserialized.is_ok() {
                deserialized.unwrap().validate();
            }
        });
    }
}
