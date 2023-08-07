#![deny(clippy::all)]

fn main() {
    let response = inverse_slice(&[12, 14, 63, 72, 55, 24], 2, 4);
    println!("{:?}", response);
}

fn inverse_slice<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    // ? Way 1
    // input
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, v)| {
    //         if i < a || i >= b {
    //             Some(v.clone())
    //         } else {
    //             None
    //         }
    //     })
    //     .collect::<Vec<T>>()

    // ? Way 2
    input
        .iter()
        .take(a)
        .chain(input.iter().skip(b))
        .cloned()
        .collect()

    // ? Way 3
    // [
    //     input.get(..a).unwrap_or(input),
    //     input.get(b..).unwrap_or(&[]),
    // ]
    // .concat()
    // .to_vec()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fixed() {
//         assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 2, 4), [12, 14, 55, 24]);
//         assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 0, 3), [72, 55, 24]);
//         assert_eq!(
//             inverse_slice(&["Intuition", "is", "a", "poor", "guide", "when", "facing", "probabilistic", "evidence"], 5, 13),
//             ["Intuition", "is", "a", "poor", "guide"]);
//         assert_eq!(inverse_slice::<i32>(&[], 1, 4), []);
//         assert_eq!(inverse_slice(&[0, 0, 0, 1, 0], 0, 3), [1, 0]);
//     }
// }
