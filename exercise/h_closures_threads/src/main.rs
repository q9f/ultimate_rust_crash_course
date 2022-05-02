use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter()
        .filter(|&x| { x % 2 == 0})
        .map(|&x| { x * x })
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let handle = thread::spawn(move ||{
        expensive_sum(my_vector)
    });
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }
    let sum = handle.join().expect("A signed integer should be provided as result.");
    println!("The child thread's expensive sum is {}", sum);
    let (tx_a, rx) = channel::unbounded();
    let tx_b = tx_a.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(300);
        tx_a.send("Thread A: 1").unwrap();
        pause_ms(400);
        tx_a.send("Thread A: 2").unwrap();
    });
    pause_ms(100);
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx_b.send("Thread B: 1").unwrap();
        pause_ms(100);
        tx_b.send("Thread B: 2").unwrap();
    });
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.
    let (tx_c, rx_c) = channel::unbounded::<&str>();
    let (tx_d, rx_d) = channel::unbounded::<&str>();
    let handle_c = thread::spawn(move || {
        for msg in rx_c {
            println!("Thread C: {}", msg);
        }
    });
    let handle_d = thread::spawn(move || {
        for msg in rx_d {
            println!("Thread D: {}", msg);
        }
    });
    for word in vec!["foo", "bar", "baz", "qux"] {
        println!("Main Thread: {}", word);
        tx_c.send(word).unwrap();
        tx_d.send(word).unwrap();
    }
    drop(tx_c);
    drop(tx_d);
    handle_c.join().unwrap();
    handle_d.join().unwrap();
    println!("Main thread: Exiting.");
}
