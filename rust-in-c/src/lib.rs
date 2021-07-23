#![crate_type = "staticlib"]
use std::ops::{Add, Mul};

#[repr(C)]
pub struct Point<'a, T> where 
    T: Add<Output = T> + Mul<Output = T> + Copy {
    
    x: &'a T,
    y: &'a T, 
}

fn slength<'a, T>(p: &Point<'a, T>) -> T where 
    T: Add<Output = T> + Mul<Output = T> + Copy {
    
    (*p.x) * (*p.x) + (*p.y) * (*p.y)
}

#[no_mangle]
pub extern fn slength_i32<'a>(p: &Point<'a, i32>) -> i32 {
    slength(p)
}