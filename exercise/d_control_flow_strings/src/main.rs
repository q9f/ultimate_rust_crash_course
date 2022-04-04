fn main() {
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
    let mut sum: i32 = 0;
    for num in 7..=23  {
        sum += num;
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count: i32 = 0;
    let mut x: i32 = 1;
    while x < 500 {
        x *= 2;
        count += 1;
    }
    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut count: i32 = 0;
    'bob: loop {
        print!("{} ", arg);
        count += 1;
        if count < 8 {
            continue 'bob;
        }
        break 'bob;
    }
    println!();
}
