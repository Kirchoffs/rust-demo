#[cfg(test)]
mod test {
    use std::thread;

    #[test]
    fn static_value() {
        let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
        
        // No move happens here
        // Give the threads a reference to the data
        thread::spawn(move || dbg!(x));
        thread::spawn(move || dbg!(x));
    }
}
