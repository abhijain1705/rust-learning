fn main() {
    println!("Hello, world!");
    let _num: u8 = 255;
    let mut string_value: String = String::from("value");
    string_value.push_str("what's up");
    println!("my value, {}", string_value);


    let employee_info:(&str, u8) = ("abhi jain", 21);
    let (employee_name, age) = employee_info;
    println!("bgvrfcd, {}, {}", employee_name,age);
    let ans= add(255, 255);
    println!("gbtvrfce, {}", ans);


    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!");

        let s: String = String::from("hello");  // s comes into scope

    takes_ownership(s);            
              

    makes_copy(s);                  
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: String) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn add(num1:u8, num2: u8)->u16 {
    // Convert num1 and num2 to u16 before addition
    (num1 as u16) + (num2 as u16)

}