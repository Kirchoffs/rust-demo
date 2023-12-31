use std::cmp::max;

#[test]
fn largest_prime_factor() {
    let mut n: i64 = 600851475143;
    let mut i = 2;
    let mut res = 1;
    while i * i <= n {
        if n % i == 0 {
            res = max(res, i);
            while n % i == 0 {
                n /= i;
            }
        } else {
            i += 1;
        }
    }
    res = max(res, n);
    assert_eq!(n, 6857);
}
