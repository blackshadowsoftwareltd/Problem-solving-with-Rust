fn main() {
    let x = validate_pin("1234");
    println!("{:?}", x);
}
fn validate_pin(pin: &str) -> bool {
    if pin.chars().all(|c| c.is_ascii_digit()) && (pin.len() == 4 || pin.len() == 6) {
        return true;
    }
    false
}

// ATM machines allow 4 or 6 digit PIN codes and PIN codes cannot contain anything but exactly 4 digits or exactly 6 digits.

// If the function is passed a valid PIN string, return true, else return false.

// Examples (Input --> Output)
// "1234"   -->  true
// "12345"  -->  false
// "a234"   -->  false
