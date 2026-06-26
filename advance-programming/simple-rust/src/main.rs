
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
    println!("Max value : {MAX_VALUE}");

    // Code block

    let code_block_value = {
        format!("Hello Everyone")
    };

    println!("Code block Value : {code_block_value}");

    let num = 10;
    print_number(num);

    // num += 12;
    println!("Number : {num}");

    // Tuple with Function
    let (num1,num2,num3) = tuple_value(1,4, 8);
    println!("num1 : {num1} | num2 : {num2} | num3 : {num3}");

    // Shadowing 
    let x = 13;
    let x_value = shadowing_fun(x);
    print!("X Value in Shadowing : {x_value}");

} 

// Mutability in Function param
fn print_number(mut x:i32) {
    x += 10;
    println!("X : {x}");
}

fn tuple_value(num1:i32, num2:i32, num3:i32) -> (i32,i32,i32) {
    (num1 + num2 + num3, num1 * num2 * num3, (num1 + num2 + num3) / 3)
}

// Shadowing Function
fn shadowing_fun(x: i32) -> i32 {
    let mut x = x;
    x += 30;
    x
}

