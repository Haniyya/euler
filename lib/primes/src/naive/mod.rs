pub fn is_naive_prime(n: usize) -> bool {
    match n {
        0 ... 1 => return false,
        2 ... 3 => return true,
        _ => {
            if n % 2 == 0  || n % 3 == 0 {
                return false;
            } else {
                let mut i = 5;

                while i * i <= n {
                    if n % i == 0 || n % (i+2) == 0 {
                        return false;
                    }
                    i += 6;
                }
            }
        }
    }
    true
}

#[test]
fn below_two(){
    assert!(!is_naive_prime(0));
    assert!(!is_naive_prime(1))
}

#[test]
fn actual_prime(){
    let big = 2 * 3 * 5 * 7 * 11 + 1;
    assert!(is_naive_prime(17));
    assert!(is_naive_prime(big))
}



