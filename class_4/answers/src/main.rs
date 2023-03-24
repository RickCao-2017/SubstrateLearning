mod ans;
use ans::ans::*;

fn main() {
    let nums = &[1u32, 2u32, 3u32];
    match sum_u32(nums) {
        Some(val) => println!("Sum is: {}", val),
        None => println!("Overflow occurred!"),
    }
}
