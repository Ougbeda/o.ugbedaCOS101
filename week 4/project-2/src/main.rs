use std::io;

fn main() {
    println!("Quadratic Roots Calculator");

    println!("Enter a:");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Input not a number");

    println!("Enter b:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Input not a number");

    println!("Enter c:");
    let mut input3 = String::new();
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Input not a number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("Root 1: {}", root1);
        println!("Root 2: {}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots");
    }
}