fn main() {
    let data = "Hello World";
    make_upper_case(data);
}
fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}
