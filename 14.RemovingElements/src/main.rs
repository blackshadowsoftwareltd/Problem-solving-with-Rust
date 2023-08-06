#![deny(clippy::all)]

fn main() {
    let response = remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("{:?}", response);
}

fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for (i, v) in arr.iter().enumerate() {
        if i % 2 == 0 {
            result.push(*v)
        }
    }
    result
}
