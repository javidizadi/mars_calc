use std::io;
fn main() {
    println!("Enter your weight in KG : ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let weight: f32 = user_input.trim().parse().unwrap();
    let mars_weight = calculate_on_mars(weight);
    println!("Weight on Mars = {}Kg", mars_weight);
}
fn calculate_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}
