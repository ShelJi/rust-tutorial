const ONE_MINUTE: i32 = 60;

// let x = 5; // let cannot be used as a global scope

fn main() {
    // Immutable variable (default)
    println!("------------------");
    println!("IMMuttable in Rust");
    println!("------------------");

    let x: i32 = 5;
    println!("The value of x is: {}", x);
    // x = 6; // ❌ This would cause a compile-time error
    println!("The value of x is: {}", x);

    // Muttable variable
    println!("------------------");
    println!("Muttable in Rust");
    println!("------------------");

    // Shadowing x with a new mutable variable
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    x += 1;
    println!("The value of x is: {x}");

    println!("------------------");
    println!("Constants in Rust");
    println!("------------------");

    // Constant variable
    // i32 -> integer of 32 bits
    // f64 -> floating point number of 64 bits
    const ONE_HOUR: i32 = ONE_MINUTE * 60;

    println!("one hour is {ONE_HOUR} seconds");

    // Variable shadowing
    println!("------------------");
    println!("Variable shadowing in Rust");
    println!("------------------");

    let n: i32 = 1;
    {
        let n: i32 = 2;
        println!("The value of n in the inner scope is: {}", n);
    }

    println!("The value of n in the outer scope is: {}", n);

    {
        println!(
            "The value of n in the inner scope without declaring is: {}",
            n
        );
    }

    println!("------------------");
    println!("Variable Declarations for scalability");
    println!("------------------");

    let spaces: &'static str = "         ";
    let length_of_spaces: usize = spaces.len();
    println!("The length of spaces is: {length_of_spaces}");

    let spaces: &'static str = "         "; // type str
    let spaces: usize = spaces.len(); // // type int
    println!("The length of spaces is: {spaces}");
    // cause no problem works fine

    // let mut spaces = "         "; // type str
    // let spaces = spaces.len(); // // type int
    // println!("The length of spaces is: {spaces}");
    // // cause type error
}
