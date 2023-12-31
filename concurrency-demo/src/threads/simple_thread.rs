use std::{thread, time};
use rand::Rng;

#[test]
fn simple_thread_without_join() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

#[test]
fn simple_thread_with_join() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Hello from the main thread.");
}

#[test]
fn simple_thread_closure() {
    let nums = vec![1, 2, 3];

    thread::spawn(move || {
        for num in nums {
            println!("number {} from the spawned thread", num);
        }
    }).join().unwrap();
}

fn f() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1 ..= 5);
    thread::sleep(time::Duration::from_micros(random_number));
    println!("Hello from another thread");
    let id = thread::current().id();
    println!("This is my thread id: {:?}", id)
}
