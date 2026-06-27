
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
    print!("X Value in Shadowing : {x_value}\n");

    condition_fun();
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

fn condition_fun() {
    // Simple Condition 
    let mark = 70;
    let mut grade = 'A';

    if mark >= 90 {
        grade = 'A';
    } else if mark >= 80 {
        grade = 'B';
    } else if mark >= 70 {
        grade = 'C';
    } else if mark >= 50 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("Mark : {mark} | Grade : {grade}");

    // Short form Condition
    let mark = 90;
    let grade = if mark >= 90 {
        'A'
    } else if mark >= 80 {
        'B'
    } else if mark >= 70 {
        'C'
    } else if mark >= 50 {
        'D'
    } else {
        'F'
    };

    println!("Mark : {mark} | Grade : {grade}");
}

// Match Condition [Like Switch Case] 
fn match_fun() {
    let marks = 90;
    let mut grade = 'N';

    match marks {
        90..=100 => grade = 'A', // condition [Pattern Matching] => after match code need to run
        80..=89 => grade = 'B',
        50..=79 => grade = 'C',
        _=> grade = 'F', // if not matched [default]
    }

    println!("Mark : {marks} | Grade : {grade}");
}






