#[cfg(test)]
mod tests {
    use std::{thread, sync::Arc};

    #[test]
    fn share_ownership() {
        use std::rc::Rc;

        let a = Rc::new([1, 2, 3]);
        let b = a.clone();
        
        println!("a: {:?}, b: {:?}", a.as_ptr(), b.as_ptr());
        assert_eq!(a.as_ptr(), b.as_ptr());
    }

    #[test]
    fn arc_share_ownership() {
        let a = Arc::new([1, 2, 3]);

        let b = a.clone();

        thread::spawn(move || {
            dbg!(b);
        });

        dbg!(a);
    }

    #[test]
    fn arc_share_ownership_with_shadow() {
        let a = Arc::new([1, 2, 3]);

        thread::spawn({
            let a = a.clone();
            move || {
                dbg!(a);
            }
        });

        dbg!(a);
    }
}
