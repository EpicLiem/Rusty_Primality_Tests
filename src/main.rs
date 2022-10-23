use std::io;

mod miller_robin;
mod baillie_psw;
mod brute_force;

fn main() {
    println!( " (1)Miller-Rabin(Fastest) \n (2)Baillie-PSW(Pretty Slow and Very Buggy) \n (3)Brute Force(100% Accurate but very slow)");
    print!("(1-3): ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: i32 = input.trim().parse().expect("Please type a number!");
    if choice == 1 {
        miller_robin::main();
    } else if choice == 2 {
        baillie_psw::main();
    } else if choice == 3 {
        brute_force::main();
    } else {
        println!("Invalid choice");
    }

}