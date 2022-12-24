// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// If is an expression not statement, expressions return values while statements don't.
// msg = if num == 5 { "Number is 5" } else { "Number is not 5" };

// Unconditional loop
// Myloop is the label for the loop.
// 'myloop: loop {
//  loop {
//     break 'myloop; // similar to continue.
//  }
//  println!("again!");
// }

// While loop
// same as lopp, but with a condition. When it evaluates to false, the loop stops.
// It is also a syntax sugar as it can be written as a loop with a condition.

// For loop
// for element in [1,2,3].iter() {...}
// for (x,y) in [(1,2), (3,4)] {...} // Array with tuples
// for num in 0..10 {...} // Range from 0 to 9
// for num in 0..=10 {...} // Range from 0 to 10

pub fn learning_control_flow() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;
    for num in 7..=23 {
        sum += num;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut count = 0;
    loop {
        if count == 8 {
            break;
        }
        print!("{} ", arg);
        count += 1;
    }

    println!();
}
