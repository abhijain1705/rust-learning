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
}

fn add(num1:u8, num2: u8)->u16 {
    // Convert num1 and num2 to u16 before addition
    (num1 as u16) + (num2 as u16)

}