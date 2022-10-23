fn fermat(n : i128) {

    if n % 2 == 0 {
        println!("{} is even", n);
        return;
    }
    if n % 3 == 0 {
        println!("{} is divisible by 3", n);
        return;
    }
    if n % 5 == 0 {
        println!("{} is divisible by 5", n);
        return;
    }
    if n % 7 == 0 {
        println!("{} is divisible by 7", n);
        return;
    }

    let p = n -1;
    let r :i128 = rand::thread_rng().gen_range(2..n);
    let a = r % p + 1;
    let mut x :i128  = 1;
    for _ in 0..p {
        x *= a;
        x %= n;
    }
    x = x % n;
    if x == 1 {
        println!("{} is prime, {}", n, x,);
    } else {
        println!("{} is composite, {}", n, x);
    }
}

pub(crate) fn main() {
    print!("Enter a number to check if it is prime: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let prime_check: i128 = input.trim().parse().expect("Please type a number!");
    let now = Instant::now();
    fermat(prime_check);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}",elapsed)

}
