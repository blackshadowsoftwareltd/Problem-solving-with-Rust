fn main() {
    let x = get_positions(666);
    println!("{:?}", x);
}
//? shortest solution
fn get_positions(step: u32) -> [u32; 3] {
    println!("{:?} {:?} {:?}", step % 3, step / 3 % 3, step / 9 % 3);
    [step % 3, step / 3 % 3, step / 9 % 3]
}
//? my solution
// fn get_positions(step: u32) -> [u32; 3] {
//     let mut hands = [0, 0, 0];
//     if step == 0 {
//         return hands;
//     }
//     for _ in 0..step {
//         if hands[0] == 2 {
//             hands[0] = 0;
//             if hands[0] == 0 && hands[1] == 2 {
//                 hands[1] = 0;
//             } else {
//                 hands[1] += 1;
//             }
//         } else {
//             hands[0] += 1;
//         }
//         if hands[0] == 0 && hands[1] == 0 && hands[2] == 2 {
//             hands[2] = 0;
//         } else if hands[0] == 0 && hands[1] == 0 {
//             hands[2] += 1;
//         }
//     }
//     hands
// }

// Imagine that you have an array of 3 integers each representing different person. Each number can be 0, 1, or 2 which represents the number of hands that person is holding up.

// Now imagine there is a sequence which follows these rules:

// None of the people have their arms raised at first
// Firstly, a person raises 1 hand; then they raise the second hand; after that they put both hands down - these steps form a cycle
// Person #1 performs these steps all the time, person #2 advances only after person #1 puts their hands down, and person #3 advances only after person #2 puts their hands down
// The first 10 steps of the sequence represented as a table are:

// Step   P1   P2   P3
// --------------------
//  0     0    0    0
//  1     1    0    0
//  2     2    0    0
//  3     0    1    0
//  4     1    1    0
//  5     2    1    0
//  6     0    2    0
//  7     1    2    0
//  8     2    2    0
//  9     0    0    1
