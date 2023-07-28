fn main() {
    let x = get_char(65);
    println!("{:?}", x);
}
fn get_char(c: i32) -> char {
    c as u8 as char
}
