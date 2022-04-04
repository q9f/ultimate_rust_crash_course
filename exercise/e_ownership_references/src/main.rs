fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    inspect(&arg);
    change(&mut arg);
    println!("I have many {}.", arg);
    if eat(arg) {
       println!("Might be bananas!?!");
    } else {
       println!("Not bananas!!!");
    }
    println!("1 + 2 = {}, even via references", add(&1, &2));
}

fn inspect(s : &String) {
    if s.ends_with("s") {
        println!("{} is plural!", s);
    } else {
        println!("{} is singular!", s);
    }
}

fn change(s: &mut String) {
    if s.ends_with("s") {
        return;
    }
    s.push_str("s");
}

fn eat(s: String) -> bool {
    if !(s.starts_with("b") && s.contains("a")) {
        return false;
    }
    true
}

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}
