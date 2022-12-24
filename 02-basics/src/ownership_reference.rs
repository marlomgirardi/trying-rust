// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

// let s1 = String::from("hello");
// let s2 = s1;
// The catch here if you are used with more high level languages is that you do not point to the same memory location.
// In Rust, there will always be a owner, and only one owner. In that case, once you do `s2 = s1` you are not saying "s2 now has the same reference as s1". Instead, you are saying, I'm passing the ownership of s1 to s2. So, s1 is no longer valid. There is a concept called borrow which I still need to see but seems like it would behave more like a high level PL. You can also `clone` the value, which will create a new memory location and copy the value to it.

// Terms:
// - Copy: Used in Rust for when only the stack memory is copied
// - Clone: Used in Rust for when the heap memory is copied

// EOF FLow (Drop, not used):
//- Destructor
//- Free heap
//- Pop stack

pub fn learning_owref() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}
