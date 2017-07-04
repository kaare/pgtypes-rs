#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::ffi::CString;

impl From<i32> for numeric {
    fn from (v: i32) -> numeric {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {PGTYPESnumeric_from_int(v, &mut num1)};
        num1
    }
}

impl From<f64> for numeric {
    fn from (v: f64) -> numeric {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {PGTYPESnumeric_from_double(v, &mut num1)};
        num1
    }
}

impl<'a>  From<&'a str> for numeric {
    fn from (v: &str) -> numeric {
        // let mut c = v.as_bytes() as &mut [i8];
        let c = CString::new(v).unwrap().into_raw();
        let mut tmp: *mut i8 = 0 as *mut i8;
        let num1 = unsafe {*PGTYPESnumeric_from_asc(c, &mut tmp)};
        num1
    }
}

impl Add<numeric> for numeric {
    type Output = Self;

    fn add (self, mut v: numeric) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_add(&mut num3, &mut v, &mut num1);
        };
        num1
    }
}

impl AddAssign<numeric> for numeric {
    fn add_assign (&mut self, mut v: numeric) {
        unsafe {
            PGTYPESnumeric_add(self, &mut v, self);
        };
    }
}

impl Sub<numeric> for numeric {
    type Output = Self;

    fn sub (self, mut v: numeric) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_sub(&mut num3, &mut v, &mut num1);
        };
        num1
    }
}

impl SubAssign<numeric> for numeric {
    fn sub_assign (&mut self, mut v: numeric) {
        unsafe {
            PGTYPESnumeric_sub(self, &mut v, self);
        };
    }
}

impl Mul<numeric> for numeric {
    type Output = Self;

    fn mul (self, mut v: numeric) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_mul(&mut num3, &mut v, &mut num1);
        };
        num1
    }
}

impl MulAssign<numeric> for numeric {
    fn mul_assign (&mut self, mut v: numeric) {
        unsafe {
            PGTYPESnumeric_mul(self, &mut v, self);
        };
    }
}

impl Div<numeric> for numeric {
    type Output = Self;

    fn div (self, mut v: numeric) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_div(&mut num3, &mut v, &mut num1);
        };
        num1
    }
}

impl DivAssign<numeric> for numeric {
    fn div_assign (&mut self, mut v: numeric) {
        unsafe {
            PGTYPESnumeric_div(self, &mut v, self);
        };
    }
}

impl Add<i32> for numeric {
    type Output = Self;

    fn add (self, v: i32) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_add(&mut num3, &mut num2, &mut num1);
        };
        num1
    }
}

impl AddAssign<i32> for numeric {
    fn add_assign (&mut self, v: i32) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_add(self, &mut num2, self);
        };
    }
}

impl Sub<i32> for numeric {
    type Output = Self;

    fn sub (self, v: i32) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_sub(&mut num3, &mut num2, &mut num1);
        };
        num1
    }
}

impl SubAssign<i32> for numeric {
    fn sub_assign (&mut self, v: i32) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_sub(self, &mut num2, self);
        };
    }
}

impl Mul<i32> for numeric {
    type Output = Self;

    fn mul (self, v: i32) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_mul(&mut num3, &mut num2, &mut num1)
        };
        num1
    }
}

impl MulAssign<i32> for numeric {
    fn mul_assign (&mut self, v: i32) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_mul(self, &mut num2, self);
        };
    }
}

impl Div<i32> for numeric {
    type Output = Self;

    fn div (self, v: i32) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_div(&mut num3, &mut num2, &mut num1)
        };
        num1
    }
}

impl DivAssign<i32> for numeric {
    fn div_assign (&mut self, v: i32) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_int(v, &mut num2);
            PGTYPESnumeric_div(self, &mut num2, self);
        };
    }
}

impl Add<f64> for numeric {
    type Output = Self;

    fn add (self, v: f64) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_add(&mut num3, &mut num2, &mut num1);
        };
        num1
    }
}

impl AddAssign<f64> for numeric {
    fn add_assign (&mut self, v: f64) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_add(self, &mut num2, self);
        };
    }
}

impl Sub<f64> for numeric {
    type Output = Self;

    fn sub (self, v: f64) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_sub(&mut num3, &mut num2, &mut num1);
        };
        num1
    }
}

impl SubAssign<f64> for numeric {
    fn sub_assign (&mut self, v: f64) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_sub(self, &mut num2, self);
        };
    }
}

impl Mul<f64> for numeric {
    type Output = Self;

    fn mul (self, v: f64) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_mul(&mut num3, &mut num2, &mut num1)
        };
        num1
    }
}

impl MulAssign<f64> for numeric {
    fn mul_assign (&mut self, v: f64) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_mul(self, &mut num2, self);
        };
    }
}

impl Div<f64> for numeric {
    type Output = Self;

    fn div (self, v: f64) -> Self {
        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        let mut num3 = self;
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_div(&mut num3, &mut num2, &mut num1)
        };
        num1
    }
}

impl DivAssign<f64> for numeric {
    fn div_assign (&mut self, v: f64) {
        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
        unsafe {
            PGTYPESnumeric_from_double(v, &mut num2);
            PGTYPESnumeric_div(self, &mut num2, self);
        };
    }
}

impl numeric {
//    pub fn div(self, v: i32) -> numeric {
//        let mut num1: numeric = unsafe { *PGTYPESnumeric_new() };
//        let mut num2: numeric = unsafe { *PGTYPESnumeric_new() };
//        let mut num3 = self;
//        let mut res = unsafe { PGTYPESnumeric_from_int(v, &mut num2) };
//        res = unsafe { PGTYPESnumeric_div(&mut num3, &mut num2, &mut num1) };
//        num1
//    }
    pub fn str(self) -> String {
        use std::ffi::CString;

        let mut num3 = self;
        let res = unsafe { CString::from_raw(PGTYPESnumeric_to_asc(&mut num3, 2)).into_string().unwrap() };
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use libc::c_char;
    use std::ffi::CString;

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

    #[test]
    fn pg_date() {
        unsafe {
            let dt_str = CString::new("1959-4-11").unwrap().into_raw();
            let mut tmp: *mut c_char = 0 as *mut c_char;
            let dt1: date = PGTYPESdate_from_asc(dt_str, &mut tmp);
            let mut dow = PGTYPESdate_dayofweek(dt1);
            println!("dow: {}", dow);

            // let mut iv1: interval = *PGTYPESinterval_new();
            let mut ts1: timestamp = PGTYPEStimestamp_from_asc(dt_str, &mut tmp);
            let iv_str = CString::new("58 years 3 weeks").unwrap().into_raw();
            let mut iv1: interval = *PGTYPESinterval_from_asc(iv_str, &mut tmp);
            let res = PGTYPEStimestamp_add_interval(&mut ts1, &mut iv1, &mut ts1);
            println!("add res: {}", res);
            let dt2 = PGTYPESdate_from_timestamp(ts1);
            dow = PGTYPESdate_dayofweek(dt2);
            println!("dow: {}", dow);

            // let a: std::os::raw::c_char = *PGTYPESdate_to_asc(dt2);
            let a = CString::from_raw(PGTYPESdate_to_asc(dt2));
            let b = a.into_string().unwrap();
            println!("date: {}", b);
        }
    }
}
