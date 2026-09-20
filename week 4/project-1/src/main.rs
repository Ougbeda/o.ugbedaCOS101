use std::io;

fn main() {
    println!("Enter employee experience (yes/no):");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    println!("Enter employee age:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    let age: i32 = input2.trim().parse().expect("Input not a number");
    let experienced = input1.trim().to_lowercase();

    if experienced == "yes" && age >= 40 {
        println!("Annual incentive: N1,560,000");
    } else if experienced == "yes" && age >= 30 && age <= 39 {
        println!("Annual incentive: N1,480,000");
    } else if experienced == "yes" && age < 28 {
        println!("Annual incentive: N1,300,000");
    } else {
        println!("Annual incentive: N100,000");
    }
}
