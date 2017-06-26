
extern crate pgtypes;


#[cfg(test)]
mod tests {
use pgtypes::numberic;
    #[test]
fn numeric() {
    println!("test");
    let a = numberic(22);
    println!("Now {:?} will print!", a);
    let b = a.div(7);
    println!("b {:?} will print!", b);
    println!("b: {}", b.str());
}
}
