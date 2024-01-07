#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn static_value() {
        static X: [i32; 3] = [1, 2, 3];

        thread::spawn(|| dbg!(&X));
        thread::spawn(|| dbg!(&X));
    }
}
