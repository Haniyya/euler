fn main() {
    let mut a = 1u64;
    let mut b = 2u64;
    let mut numbers: Vec<u64> = vec![a,b];
    let mut sum = 2u64;

    while a + b <= 4000000 {
        let c = a + b;
        numbers.push(c);
        if c % 2 == 0 { sum = sum + c }
        a = numbers[numbers.len() - 2];
        b = numbers[numbers.len() - 1];
    }

    println!("{}", sum)
}
