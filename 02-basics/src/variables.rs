// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

pub fn learning_variables() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;
    println!("{} missiles left", missiles);
}
