extern crate libc;

#[derive(Debug)]
#[repr(C)]
struct Point<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[link(name = "math")]
extern {
    fn slength(input: &Point) -> libc::c_long;
}

fn main() {
    let input = Point { x: &4, y: &5 };
    let output = unsafe { slength(&input) };
    println!("length of {:?} is {}", input, output);
}