// use std::{fmt::Error, fs::File, io::{self, Read}};

use std::fs::File;
// use std::fs::{self, File};
use std::io::{Error};
use std::io::Read;

fn main() -> Result<(), Error>  {
    let file_name: &str = "../text.txt";

    let mut file = File::open(file_name)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("File Content: \n{}", content);

    exception::main::shiow();

    Ok(())
}

// fn read_file() -> Result<(), Error> {
//     let file_path: &str = "../text.txt";
//     let contents = fs::read(file_path)?;
//     println!("Content (bytes): {:?}", contents);
//     Ok(())
// }

// fn ternary_operator() {

// }


// fn result_test() -> Result<(), Error> {
//     let file:Result<File, std::io::Error>= File::open("../text.txt");
//     match file {
//         Ok(f) => {
//             println!("File Can Open");
//         },
//         Err(f) => {
//             println!("File can't open");
//         }
//     }
//     Ok(())
// }

