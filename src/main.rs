use std::io;

mod miller_robin;
mod baillie_psw;
mod brute_force;

fn main() {
    println!( "(1)Miller-Rabin \n (2)Baillie-PSW \n (3)Brute Force");
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