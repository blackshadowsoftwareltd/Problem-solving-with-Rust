#![deny(clippy::all)]

fn main() {
    let response = odd_or_even(vec![0, 1, 5]);
    println!("{:?}", response);
}

fn odd_or_even(numbers: Vec<i32>) -> String {
    let sum: i32 = numbers.iter().sum();
    match sum {
        sum if sum % 2 == 0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}
