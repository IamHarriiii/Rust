// 1. 🌟 A variable can be used only if it has been initialized.
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5;
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
    
    main1()
}


// 2. 🌟 Use mut to mark a variable as mutable.
// Fill the blanks in the code to make it compile
fn main1() {
    let mut  x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}