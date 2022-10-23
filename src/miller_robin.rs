use std::{io::stdin};
use std::io;
use rand::{Rng};
use std::time::Instant;
use num::bigint::{BigUint, ToBigUint, BigInt, RandBigInt};
use num::traits::cast::ToPrimitive;

//Miller Rabin Primality Test

// func that does x^y % p
fn power(x: i128, y: i128, p: i128) -> BigInt {
    let mut res :BigInt = BigInt::from(1);
    let mut x :BigInt = BigInt::from(x % p);
    let mut y :BigInt = BigInt::from(y);
    while y > BigInt::from(0) {
        if &y % 2 == BigInt::from(1){
            res = (res * &x) % p;
        }
        y /= 2;
        x = (&x * &x) % p;
    }
    return res;
}

fn miller_rabin(n :i128, i :i128) -> bool{
    //check these first
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n % 3 == 0 {
        return false;
    }
    // find d such that n-1 = 2^d * r
    let mut d = BigInt::from(n - 1);
    while &d % 2 == BigInt::from(0) {
        d /= 2;
    }

    // perform i iterations of the test
    for ii in 0..i {
        // pick a random number a 1 between n-2
        let mut rng = rand::thread_rng();
        let a:i128  = 2 + rng.gen_range(0..n - 2);
        // compute x = a^d % n
        let mut x = power(a, (n - 1), n);
        // if x is 1 or n-1, continue to next iteration
        if x == BigInt::from(1) || x == &n - BigInt::from(1) {
            if ii == i -1 {
                return true;
            }
            continue;
        }

        while d != BigInt::from(n - 1) {
            x = (&x * &x) % n;
            d = d * 2;
            // if x is 1, return composite
            if x == BigInt::from(1) {
                return false;
            }
            // if x is n-1, continue to next iteration
            if x == BigInt::from(n - 1) {
                if ii == i - 1 {
                    return true;
                }
                continue;
            }
        }
    }
    // return composite
    return false;
}

pub(crate) fn main(){
    // check input for number to be checked and parse it
    print!("Enter a number to check if it is prime: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut prime_input = String::new();
    stdin().read_line(&mut prime_input).expect("Failed to read line");
    let prime_check: i128 = prime_input.trim().parse().expect("Please type a number!");
    // check input for number of iterations and parse it
    let mut iteration_input = String::new();
    print!("Enter the number of iterations: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    stdin().read_line(&mut iteration_input).expect("Failed to read line");
    let iteration_check: i128 = iteration_input.trim().parse().expect("Please type a number!");
    // start timer
    let now = Instant::now();
    // run miller rabin test and print result
    if miller_rabin(prime_check, iteration_check) {
        println!("{} is prime", prime_check);
        println!("Elapsed: {:.2?}", now.elapsed());
    } else {
        println!("{} is composite", prime_check);
        println!("Elapsed: {:.2?}", now.elapsed());
    }
}