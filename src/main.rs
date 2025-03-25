use std::env;

fn gcd( a: u128, b: u128) -> u128 {
    
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
    // while b > 0 {
    //     a = b;
    //     b = a % b;
    // }
    // a
}
#[allow(dead_code)]
/// This function is to calculate the factorial of  a number
fn factorial(n: u128) -> u128 {
    let mut fact = 1;
    for i in 1..=n {
        fact *= dbg!(i);
    }
    fact
}
#[test]
fn test_factorial() {
    assert_eq!(24, factorial(4))
}

#[allow(dead_code)]
fn fizzbuzz(a: u32) -> u32 {
    let _ = a;
    a
}

#[allow(dead_code)]
///The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:
///- If ni is 1, then the sequence terminates at ni.
///- If ni is even, then ni+1 = ni / 2.
///- If ni is odd, then ni+1 = 3 * ni + 1.
fn collatz_sequence(mut n1: u128) -> u128 {
    let mut arr = 1;
    while n1 > 1 {
        n1 = if n1 % 2 == 0 { n1 / 2 } else { 3 * n1 + 1 };
        arr += 1;
    }
    arr
}
fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        let arg: u64 = arg.trim().parse().expect("Error parsing argument");
        numbers.push(arg);
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d.into(), (*m).into()) as u64;
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
}
