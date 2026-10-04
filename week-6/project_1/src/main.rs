use std::io;

fn main() {
    println!("Welcome to PAU Cafeteria!");
    println!("Here is our menu:");
    println!("P: Poundo Yam / Edinkaiko Soup - ₦3,200");
    println!("F: Fried Rice & Chicken - ₦3,000");
    println!("A: Amala & Ewedu Soup - ₦2,500");
    println!("E: Eba & Egusi Soup - ₦2,000");
    println!("W: White Rice & Stew - ₦2,500");

    // Read food type
    println!("\nEnter the letter of your choice:");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim();

    // Read quantity
    println!("Enter quantity:");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
    let quantity: i32 = quantity_input.trim().parse().expect("Please enter a valid number");

    // Prices
    let p_price = 3200;
    let f_price = 3000;
    let a_price = 2500;
    let e_price = 2000;
    let w_price = 2500;

    // Compute total
    let mut total = 0;
    if food_type == "P" {
        total = p_price * quantity;
    } else if food_type == "F" {
        total = f_price * quantity;
    } else if food_type == "A" {
        total = a_price * quantity;
    } else if food_type == "E" {
        total = e_price * quantity;
    } else if food_type == "W" {
        total = w_price * quantity;
    } else {
        println!("Invalid choice!");
        return;
    }

    // Discount if total > 10,000
    if total > 10000 {
        let discount = (total as f32) * 0.05;
        let final_total = (total as f32) - discount;
        println!("\nTotal before discount: ₦{}", total);
        println!("Discount applied: ₦{}", discount);
        println!("Final total: ₦{}", final_total);
    } else {
        println!("\nTotal charge: ₦{}", total);
    }

    println!("\nThank you for dining with us at PAU Cafeteria!");
}
