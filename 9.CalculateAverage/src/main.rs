#![deny(clippy::all)]

fn main() {
    let vectors = vec![
        17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
    ];
    let response = find_average(&vectors);
    println!("{:?}", response);
}

fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for &item in slice {
        sum += item;
    }
    sum / slice.len() as f64
}
