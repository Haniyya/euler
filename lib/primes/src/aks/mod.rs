use std::iter::repeat;

fn aks_coefficients(k: usize) -> Vec<i64> {
    let mut coefficients = repeat(0i64).take(k+1).collect::<Vec<_>>();
    coefficients[0] = 1;
    for i in  1..(k+1) {
        coefficients[i] = -(1..i).fold(coefficients[0], |prev,j| {
            let old = coefficients[j];
            coefficients[j] = old - prev;
            old
        });
    }
    coefficients
}

pub fn is_aks_prime(p: usize) -> bool {
    if p < 2 {
        false
    } else {
        let c = aks_coefficients(p);
        (1 .. (c.len() - 1) / 2 + 1).all(|i| (c[i] % (p as i64)) == 0)
    }
}

#[test]
fn correct_coeffs(){
    let vec = vec![3 as i64,3 as i64];
    assert_eq!(vec, aks_coefficients(3))
}

#[test]
fn correct_num_of_coeffs(){
    assert_eq!(2, aks_coefficients(3).len())
}

#[test]
fn small_prime() {
    assert!(is_aks_prime(11));
    assert!(is_aks_prime(43));
}

#[test]
fn below_two() {
    assert!(!is_aks_prime(0));
    assert!(!is_aks_prime(1));
}

#[test]
fn big_prime() {
    let big = 541;
    assert!(is_aks_prime(big))
}
