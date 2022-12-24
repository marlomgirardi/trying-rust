// Scalar Types

// Integer
// u{size} = Unsigned (8, 16, 32, 64, 128, size)
// i{size} = Signed (8, 16, 32, 64, 128, size)
// TODO: look a bit later at usize and isize

// Float
// f{size} = Float (32, 64)
// 64 is default and more precise, but 32 is faster and can be very slow on 32bit systems
// IEEE-754 standard

// Char
// char = Character (4 bytes, UCS-4 / UTF-32)
// Can be any character, for any alphabet, including emojis.

// Compound Types

// Tuple `let info = (1, "hello", 3.14);`
// You can use dot notation `info.0` or use destructuring `let (x, y, z) = info`
// Array `let arr = [1, 2, 3, 4, 5];`
// `let arr = [0; 3];` ";" is separator to how many items in the array.
// `let buf: [u8; 3] = [0; 3];` can also be used to specify the type of the array.
// Arrays are fixed length, and can only contain one type, and their max size is 32bit.

// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use basics::*;

pub fn learning_types() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}
