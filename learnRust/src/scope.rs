// 3. A scope is the range within the program for which the item is valid.

// 🌟
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

// 🌟🌟

// Fix the error with the use of define_x
pub fn scope_example() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String {
    main();
    let x = String::from("hello");
    return x
}