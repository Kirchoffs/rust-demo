#[cfg(test)]
mod tests {
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
    
    fn f() {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1 ..= 5);
        thread::sleep(time::Duration::from_micros(random_number));
        println!("Hello from another thread");
        let id = thread::current().id();
        println!("This is my thread id: {:?}", id)
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
    
    #[test]
    fn simple_thread_closre_return_value() {
        let nums = vec![1, 2, 3];
    
        let t = thread::spawn(move || {
            let len = nums.len();
            let sum = nums.iter().sum::<usize>();
            sum / len
        });
    
        let average = t.join().unwrap();
        println!("average is {}", average);
    }
    
    #[test]
    fn simple_thread_customized() {
        let nums = vec![1, 2, 3];
    
        let t = thread::Builder::new()
            .name("print_nums".to_string())
            .stack_size(1024 * 4);
    
        let t = t.spawn(move || {
            for num in nums {
                println!("number {} from the spawned thread", num);
            }
            println!("Thread {} finished", thread::current().name().unwrap());
        }).unwrap();
    
        t.join().unwrap();
    }    
}
