use std::io;

fn main() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Please enter the index of the value you want to access");
    let mut input_index = String::new();

    io::stdin().read_line(&mut input_index).expect("Failed to read line");

    let input_index: usize = input_index.trim().parse().expect("Please type a number");

    match a.get(input_index){
        Some(value) => println!("the value at index {} is {}", input_index, value),
        None => println!("value at index not found")
    }
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");
}