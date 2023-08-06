#![deny(clippy::all)]

fn main() {
    let response = valid_parentheses(")(");
    println!("{:?}", response);
}

fn valid_parentheses(parens: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in parens.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => return false,
        }
    }
    println!(":::: {:?}", stack);
    stack.is_empty()
}
