
use std::io;

fn main() {
    println!("\nStudent Information Managemnt System! ");


    println!("\nPlease Enter your name. ");
    let mut name = String::new();
      io::stdin()
      .read_line(&mut name)
      .expect("Failed to read imput");
    println!("Your name is: {}",name);


    println!("\nEnter your age.");
    let mut age = String::new();
       io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}",age);


}
