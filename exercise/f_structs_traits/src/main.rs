trait Bite {
    fn bite(&mut self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: u32,
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.amount_left -= 1;
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    let mut nibbles: i32 = 0;
    loop {
        item.bite();
        nibbles += 1;
        if nibbles < 10 {
            continue;
        }
        break;
    }
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}
