#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pg_numeric() {
        unsafe {
            let mut num1: numeric = *PGTYPESnumeric_new();
            let mut res = PGTYPESnumeric_from_int(1, &mut num1);
            println!("res: {}", res);
            let mut int1: i32 = 0;
            println!("int: {}", int1);
            res = PGTYPESnumeric_add(&mut num1, &mut num1, &mut num1);
            println!("res: {}", res);
            res = PGTYPESnumeric_to_int(&mut num1, &mut int1);
            println!("res: {}", res);
            println!("int: {}", int1);
        }
    }
}
