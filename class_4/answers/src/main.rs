mod ans;
use ans::*;

fn main() {
    // 5
    show_lights();
    println!();

    // 6 
    let nums = &[1u32, 2u32, 3u32];
    match sum_u32(nums) {
        Some(val) => println!("Sum is: {}", val),
        None => println!("Overflow occurred!"),
    }
    println!();

    // 7
    print_test()
}
