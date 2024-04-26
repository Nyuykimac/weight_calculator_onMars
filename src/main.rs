use std::io;

fn main() {
    println!("Enter the weight here: (kg)");
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input).unwrap();

    let value = input.trim().parse().unwrap();
    let weight = weight_calculator(value);
    println!("The weight in Mars is: {}", weight);

    println!("Operation completed Successfully")
}

fn weight_calculator(weight: f32) -> f32 {
    let weight_on_mars = (weight / 9.81) * 3.711;
    return weight_on_mars;
}
