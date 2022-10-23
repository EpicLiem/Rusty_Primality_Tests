use std::io;

mod miller_robin;
mod baillie_psw;
mod brute_force;

fn main() {
    let mut r = bool::default();
    println!( " (1)Miller-Rabin(Fastest) \n (2)Baillie-PSW(Pretty Slow and Very Buggy) \n (3)Brute Force(100% Accurate but very slow)");
    print!("(1-3): ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: i32 = input.trim().parse().expect("Please type a number!");
    if choice == 1 {
        r = miller_robin::main();
    } else if choice == 2 {
        r = baillie_psw::main();
    } else if choice == 3 {
        r = brute_force::main();
    } else {
        println!("Invalid choice");
    }
    if r == true {
        println!("Done");
    }
    else {
        println!("Error");
        main();
    }
}