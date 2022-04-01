const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // declare an int32 and initialize it
    let bunnies: i32 = 4;

    // initialize multiple variables at once
    let (foo, bar) = (8, 16);

    // let => immutable! => safety, concurrency, speed
    // foo = 2; // => error: cannot assign twice to immutable variable

    // declare a mutable variable
    let mut baz: i32 = 32;
    baz += bunnies;
    baz += foo;
    baz += bar;

    // constants are more immutable than variables
    // constants are screaming capital case
    const WARP_FACTOR: f64 = 9.9;

    println!("Hello, world! {}, {}!", baz, WARP_FACTOR);

    // actual exercise (A)
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles ...", ready, missiles);

    missiles = missiles - ready + ready;
    println!("{} missiles left ...", missiles - ready);
}
