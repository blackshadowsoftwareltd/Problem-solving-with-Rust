fn main() {
    let x = enough(10, 5, 5);
    println!("{}", x);
}
fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    if cap >= on + wait {
        0
    } else {
        on + wait - cap
    }
}

// cap is the amount of people the bus can hold excluding the driver.
// on is the number of people on the bus excluding the driver.
// wait is the number of people waiting to get on to the bus excluding the driver.
