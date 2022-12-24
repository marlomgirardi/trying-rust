// struct RedFox { enemy: bool, life: u8 }
// let fox = RedFox { enemy: false, life: 100 };

// impl RedFox {
//     fn new() -> Self {
//       Self { enemy: false, life: 100 }
//     }
// }
// let fox = RedFox::new();

// trait Noise { fn get_noise(&self) -> String; }
// impl Noise for RedFox { fn get_noise(&self) -> &str { "Meow! } }
// This could be implemented directly into RedFox `impl` but traits can be used as generics so you can have a function that takes any type that implements the Noise trait.
// fn make_noise<T: Noise>(animal: T) { println!("{}", animal.get_noise()); }

// Not exactly true, but for comparison:
// - struct can be considered like classes
// - traits are like interfaces

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    amount_left: u8,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

pub fn learning_st() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot, 5);
    println!("Bunny nibbles for awhile: {:?}", carrot);

    bunny_nibbles(&mut grapes, 21);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

fn bunny_nibbles<T: Bite>(food: &mut T, bites: u8) {
    for _ in 0..bites {
        food.bite();
    }
}
