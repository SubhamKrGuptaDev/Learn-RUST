
fn main() {

    // Mutable 
    let mut x = 10;
    x = x + 2;

    println!("X Value : {x}");

    // Shadowing 
    let y = 10;
    let y = y + 40;
    println!("Y Value : {y}");

    // Use cases for Shadowing [We Can use different type with same variable]
    // let shadowing_use_case:i32 = 12;
    // let shadowing_use_case:f64 = 12.00; // Change Type
    // println!("Shadowing Use Case : {shadowing_use_case}")

    // Constants [const variable name with caps type = value {nothing is optional}]
    const MAX_VALUE:i32 = 200;
    println!("Max value : {MAX_VALUE}")


} 
