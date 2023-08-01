#![deny(clippy::all)]

fn main() {
    let response = switcheroo("aabacbaa");
    println!("{:?}", response);
}

fn switcheroo(s: &str) -> String {
    let mut result = String::new();
    for x in s.chars() {
        match x {
            'a' => result.push('b'),
            'b' => result.push('a'),
            _ => result.push(x),
        }
    }
    result
}

// Given a string made up of letters a, b, and/or c, switch the position of letters a and b (change a to b and vice versa). Leave any incidence of c untouched.

// Example:

// 'acb' --> 'bca'
// 'aabacbaa' --> 'bbabcabb'
