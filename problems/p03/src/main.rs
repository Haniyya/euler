extern crate primes;

use primes::naive::is_naive_prime;

fn biggest_prime_factor(num: u64) -> u64 {
    for i in (3..num) {
        let factor = num / i;
        println!("Testing {}", factor);
        if is_naive_prime(factor) { return factor }
    }
    0
}

fn main() {
    let num = 600851475143u64;
    println!("Is prime: {}", is_naive_prime(num));
    println!("{}", biggest_prime_factor(num))
}
