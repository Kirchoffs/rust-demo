#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn scoped_thread() {
        let nums = vec![1, 2, 3];

        thread::scope(|scope| {
            scope.spawn(|| {
                println!("length: {}", nums.len());
                println!("Thread {:?} finished", thread::current().id());
            });

            scope.spawn(|| {
                for num in &nums {
                    println!("{num}");
                }
                println!("Thread {:?} finished", thread::current().id());
            });
        });

        println!("Main thread finished");
    }
}
