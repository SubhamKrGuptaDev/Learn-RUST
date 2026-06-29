
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
    println!("Vec 3 : {:?}", result_vec)

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
