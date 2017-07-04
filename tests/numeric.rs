
extern crate pgtypes;


#[cfg(test)]
mod tests {

use pgtypes::numeric;

#[test]
    fn numeric() {
        println!("test");
        let pi = numeric::from("22.0") / 7; // Well, not really π
        let mut area = pi * 2;  // Radius is 1
        area += 4;
        println!("-------> area is: {}", area.str());
    }
}
