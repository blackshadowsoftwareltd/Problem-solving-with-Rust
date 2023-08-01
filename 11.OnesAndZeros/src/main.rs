#![deny(clippy::all)]

fn main() {
    let response = binary_slice_to_number(&[0, 0, 0, 1]);
    println!("{:?}", response);
}

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let mut x = String::new();
    for v in slice {
        x.push_str(&v.to_string());
    }
    u32::from_str_radix(&x, 2).unwrap()
}

// Given an array of ones and zeroes, convert the equivalent binary value to an integer.
//
// Eg: [0, 0, 0, 1] is treated as 0001 which is the binary representation of 1.
//
// Examples:
//
// Testing: [0, 0, 0, 1] ==> 1
// Testing: [0, 0, 1, 0] ==> 2
// Testing: [0, 1, 0, 1] ==> 5
// Testing: [1, 0, 0, 1] ==> 9
// Testing: [0, 0, 1, 0] ==> 2
// Testing: [0, 1, 1, 0] ==> 6
// Testing: [1, 1, 1, 1] ==> 15
// Testing: [1, 0, 1, 1] ==> 11
