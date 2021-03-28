// use parser::*;

// type Byte = u8;

// fn decode_to_integer(bytes: &[u8]) -> usize {
//     let mut value = 0usize;
//     for byte in bytes {
//         if *byte < 0x80 {
//             value += *byte as usize;
//             break;
//         } else {
//             use std::convert::TryInto;
//             let temp = *byte - 0x80;
//             let temp: usize = temp.try_into().unwrap();
//             value += temp;
//         }
//     }
//     value
// }

// #[test]
// fn test() {
//     assert_eq!(decode_to_integer(&[0]), 0);
//     assert_eq!(decode_to_integer(&[100]), 100);
//     assert_eq!(decode_to_integer(&[200]), 100);
// }