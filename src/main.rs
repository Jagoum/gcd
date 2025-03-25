
fn gcd(mut a: u128, mut b: u128) -> u128 { //   12 4 
    // if b > 0 {
    //     gcd(b, a % b)
    // } else {
    //     a
    // }
    while b > 0 {
        a = b;
        b = a % b;
    }
    a
}
/// This function is to calculate the factorial of  a number
fn factorial(n: u128) -> u128 {
    let mut fact = 1;
    for i in 1..=n {
        fact *= dbg!(i);
    }
    fact
}
#[test]
fn test_factorial(){
    
    assert_eq!(24, factorial(4))
}

fn fizzbuzz(a: u32) -> u32 {
    let _ = a;
    a
}
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
    let x1 = 128;
    let x2 = 7;
    dbg!(gcd(x1, x2));
    let x1 = factorial(x2);
    // gracefully handling errors
    fizzbuzz(match x1.try_into() {
        Ok(good) => good,
        Err(_) => {
            eprintln!("Please enter a u32 integer ");
            1
        }
    });

    println!(
        "Result of Collatz sequence of {x2} is {:?}",
        collatz_sequence(match x2.try_into() {
            Ok(good) => {
                good
            }
            Err(_) => {
                eprintln!("Please enter a valid number");
                return;
            }
        })
    );
}
