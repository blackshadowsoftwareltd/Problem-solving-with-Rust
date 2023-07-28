fn main() {
    let x = simple_multiplication(5);
    println!("The value of x is: {}", x);
}
fn simple_multiplication(number: u8) -> u8 {
    if number % 2 == 0 {
        number * 8
    } else {
        number * 9
    }
}
