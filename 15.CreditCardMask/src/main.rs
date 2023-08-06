#![deny(clippy::all)]

fn main() {
    let response = maskify("12345");
    println!("{:?}", response);
}

fn maskify(cc: &str) -> String {
    if cc.len() < 5 {
        return cc.to_string();
    }
    let mut result = String::new();
    let max = cc.len() - 4;
    for (i, v) in cc.chars().into_iter().enumerate() {
        if i < max {
            result.push_str("#")
        } else {
            result.push(v);
        }
    }
    result
}
