fn main() {
    let x = divisors(12);
    println!("{:?}", x);
}
fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    if is_prime(integer) {
        return Err(format!("{} is prime", integer));
    }
    let mut divs: Vec<u32> = Vec::new();
    for i in 2..integer {
        if integer % i == 0 {
            divs.push(i)
        }
    }
    Ok(divs)
}
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
