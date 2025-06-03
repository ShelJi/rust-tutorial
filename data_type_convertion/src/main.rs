fn main() {
    let str_num: &'static str = "81";
    // let str_str = "81q";

    let converted_number: i32 = str_num.parse().expect("Could not parse...");
    // let converted_number: i32 = str_str.parse().expect("Could not parse...");
    
    println!("Converted number: {}", converted_number);

}
