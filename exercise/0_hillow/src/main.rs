use hillow::greet;
use rand::Rng;

// the hillow binary
fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen();
    greet(x);
}
