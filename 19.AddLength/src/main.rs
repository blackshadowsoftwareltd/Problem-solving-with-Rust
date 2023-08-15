#![deny(clippy::all)]

fn main() {
    let response = add_length("apple ban");
    println!("{:?}", response);
}

fn add_length(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|x| format!("{} {}", x, x.len()))
        .collect()
}

// "apple ban" --> ["apple 5", "ban 3"]
// "you will win" -->["you 3", "will 4", "win 3"]
