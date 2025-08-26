// Slience some warning so they don't distract form the exercise.
#![allow(unused_mut)]

fn main() {

    let mut arg: String = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Please Supply an argument to this program.");
            std::process::exit(-1);
        })
        .to_owned();

    inspect(&arg);

    // InLine Function
    fn inspect(arg: &String) {
        if arg.ends_with('s') {
            println!("{} is plural", arg);
        } else {
            println!("{} is Singular", arg);
        }
    }

    change_arg(&mut arg);
    println!("\nAfter Change arg: {}",arg);

    fn change_arg(arg:&mut  String) {
        if !arg.ends_with('s') {
            arg.push('s');
        }
    }

    

}
