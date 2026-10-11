use std::io;

// Function for trapezium area
fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

// Function for rhombus area
fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// Function for parallelogram area
fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// Function for cube surface area
fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

// Function for cylinder volume
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    let pi: f64 = 22.0 / 7.0;
    pi * radius * radius * height
}

fn main() {
    println!("Welcome to the MTH 101 Shape Calculator!");
    println!("Choose a shape to calculate:");
    println!("1. Trapezium area");
    println!("2. Rhombus area");
    println!("3. Parallelogram area");
    println!("4. Cube surface area");
    println!("5. Cylinder volume");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: i32 = choice.trim().parse().expect("Invalid choice");

    match choice {
        1 => {
            let (height, base1, base2) = read_three("height", "base1", "base2");
            println!("Trapezium area = {}", trapezium_area(height, base1, base2));
        }
        2 => {
            let (d1, d2) = read_two("diagonal1", "diagonal2");
            println!("Rhombus area = {}", rhombus_area(d1, d2));
        }
        3 => {
            let (base, altitude) = read_two("base", "altitude");
            println!("Parallelogram area = {}", parallelogram_area(base, altitude));
        }
        4 => {
            let side = read_one("side");
            println!("Cube surface area = {}", cube_surface_area(side));
        }
        5 => {
            let (radius, height) = read_two("radius", "height");
            println!("Cylinder volume = {}", cylinder_volume(radius, height));
        }
        _ => println!("Invalid selection! Please run again."),
    }
}

// Helper functions for reading input
fn read_one(label: &str) -> f64 {
    let mut input = String::new();
    println!("Enter {}:", label);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn read_two(label1: &str, label2: &str) -> (f64, f64) {
    let a = read_one(label1);
    let b = read_one(label2);
    (a, b)
}

fn read_three(label1: &str, label2: &str, label3: &str) -> (f64, f64, f64) {
    let a = read_one(label1);
    let b = read_one(label2);
    let c = read_one(label3);
    (a, b, c)
}
