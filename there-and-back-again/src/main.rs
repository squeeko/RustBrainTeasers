use std::f32::consts::PI;

pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(item: Degrees) -> Self {
        Self(item.0 * PI / 180.0)
    }
}

fn main() {
    let one_eighty_degrees = Degrees::new(180.0);
    let one_eighty_radians: Radians = one_eighty_degrees.into();
    println!("180 Degrees in Radians = {} ", one_eighty_radians.0);
}

/*
The surprise here is that the Into trait wasn’t implemented, yet the program
was still able to use the into function with the Radians type.
When you define the From trait, Rust automatically implements the reciprocal
Into trait for you.

Using the example code from this brain teaser, you can convert degrees to
radians using either let r : Radians = d.into() or let r = Radians::from(d).

you can implement your own TryFrom trait for your types to
provide conversion with the possibility of reporting failure.
Suppose you want to constrain a numeric type to only accept values between
0 and 10; you might do something like this to implement try_from:
use std::convert::TryFrom;

struct ZeroToTen(i32);

impl TryFrom<i32> for ZeroToTen {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {

        if value < 0 || value > 10 {

            Err("Value must be between 0 and 10")

        } else {

            Ok(Self(value))

        }

    }
}

std::convert::From trait
https://doc.rust-lang.org/std/convert/trait.From.html
std::convert::TryFrom trait
https://doc.rust-lang.org/std/convert/trait.TryFrom.html
*/
