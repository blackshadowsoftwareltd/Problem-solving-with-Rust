fn main() {
    let x = get_middle("of");
    println!("{}", x);
}
fn get_middle(s: &str) -> &str {
    let m = s.len() / 2;
    if s.len() % 2 == 0 {
        s.get(m - 1..m + 1).unwrap()
    } else {
        s.get(m..m + 1).unwrap()
    }
}
//? Get the Middle Character

// fn example_tests() {
//     assert_eq!(get_middle("test"),"es");
//     assert_eq!(get_middle("testing"),"t");
//     assert_eq!(get_middle("middle"),"dd");
//     assert_eq!(get_middle("A"),"A");
//     assert_eq!(get_middle("of"),"of");
//   }
