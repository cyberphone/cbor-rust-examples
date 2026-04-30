// Error handling in a low-level decoder

use std::{num::ParseFloatError, str::FromStr};

struct Demo {
    vector: Vec<f64>
}

impl Demo {
    fn new() -> Demo {
        Demo{vector: Vec::new()}
    }

    fn convert_with_panic(f64_string: &str) -> f64 {
        match f64::from_str(f64_string) {
            Ok(value) => {
                value
            }
            Err(_e) => {
                panic!("Invalid float: {f64_string}");
            }
        }
    }

    fn convert_std(f64_string: &str) -> Result<f64, ParseFloatError> {
        f64::from_str(f64_string)
    }

    fn add_with_panic(&mut self, f64_string: &str) -> usize {
        match f64::from_str(f64_string) {
            Ok(value) => {
                self.vector.push(value);
            }
            Err(_e) => {
                panic!("Invalid float: {f64_string}");
            }
        }
        self.vector.len()
    }
}
fn main() {
    let mut a = Demo::new();
    let r1 = a.add("5.7");
    // SILENTLY FAIL
    let r2 = a.add("pretty bad")?;
    let n = a.add_with_panic("6.0");
    println!("There are {n} elements in the vector");
    let _ = a.add_with_panic("Not really a number");
}