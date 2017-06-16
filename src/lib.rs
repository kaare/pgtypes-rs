#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

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
