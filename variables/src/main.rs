const ONE_MINUTE: i32 = 60;

// let x = 5; // let cannot be used as a global scope

fn main() {
    // Immutable variable (default)
    println!("------------------");
    println!("IMMuttable in Rust");
    println!("------------------");
    
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // ❌ This would cause a compile-time error
    println!("The value of x is: {}", x);

    
    // Muttable variable
    println!("------------------");
    println!("Muttable in Rust");
    println!("------------------");

    // Shadowing x with a new mutable variable
    let mut x = 5;
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
    let n = 1;
    {
        let n = 2;
        println!("The value of n in the inner scope is: {}", n);
    }

    println!("The value of n in the outer scope is: {}", n);
}
