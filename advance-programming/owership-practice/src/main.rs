fn main() {
    // Function that take ownership
    let vec_1 = vec![1,2,3,4,5,6];
    take_owership(vec_1.clone()); // Ownership transfect to method
    println!("Vec 1 of {:?}",vec_1);

    // Function that give ownership
    let vec_2 = give_ownership();
    println!("Vec 2 : {:?}",vec_2);

    // Function that take and give ownership
    let vec_3 = vec![6,3,32,9,0,1,3,5];
    let result_vec = take_and_give_ownership(vec_3);
    println!("Vec 3 : {:?}", result_vec );


    // Borrowing 
    let mut vec_1 = vec![1,2,3,5];
    let ref_1 = &mut vec_1;

    println!("Reference Data : {:?}",ref_1);
    ref_1.push(60);
    println!("Reference Data : {:?}",vec_1);

    // Function that immutably borrow values
    let vec_1 = vec![4,6,8,90];
    let ref_1 = &vec_1;
    borrow_reference(ref_1);
    println!("Vector Value : {:?}",vec_1);


    // function that mutably borrow values
    let mut vec_1 = vec![09,45,76,132,76];
    let ref_1 = &mut vec_1;
    take_and_return_ownership(ref_1);
    println!("Orginal Vector Value : {:?}",vec_1);

    //  Functions that do not uses borrow but returns it
    // let vec_2 = vec![4,8,9,2,68,34689];
    // let ref_2 = return_reference(vec_2);
    // println!("Reference 2 : {:?}",ref_2);

    //  Functions that uses mixed type of borrow
    let mut vec_1 = vec![2,45,1211,7,3,2,64];
    let subject = "hello Subham Gupta";
    mixed_borrow(&subject, &mut vec_1);

}

fn mixed_borrow(subject: &str, vec: &mut Vec<i32>) {
    println!("Subject value '{}' Vector Value {:?}", subject, vec);
    vec.push(subject.len() as i32);
    println!("Subject value '{}' Vector Value {:?}", subject, vec);
}

// Not valid variable
// fn return_reference(vec: Vec<i32>) -> &Vec<i32> {
//     &vec
// }

// not valid variable
// fn return_reference() -> &Vec<i32> {
//     let vec = vec![33,5,2,6,2];
//     &vec
// }

fn take_and_return_ownership(vec:&mut Vec<i32>) {
    vec.push(50);
}

fn take_and_give_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(30);
    vec
}

fn give_ownership() -> Vec<i32> {
    vec![6,3,25,8,53,79,3]
}

fn take_owership(vec: Vec<i32>) {
    println!("Print Vec : {:?}",vec);
}

fn borrow_reference(vec: &Vec<i32>) {
    println!("Reference of Vector : {:?}",vec);
}


