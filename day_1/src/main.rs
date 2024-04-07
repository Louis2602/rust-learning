// fn main() {
//     let x: i32 = 5; // integer type
//     let _y: i32; // use _y to mark as unused var

//     assert_eq!(x, 5);
//     println!("Success!");
// }

// =======================================================
// use `mut` to mark a variable as mutable

// fn main() {
//     let mut x = 1;
//     x += 2;

//     assert_eq!(x, 3);
//     println!("Success")
// }

// =======================================================
// Scope

// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 5;

//     {
//         println!("The value of x is {} and value of y is {}", x, y);
//     }

//     println!("The value of x is {} and value of y is {}", x, y);
// }

// =======================================================
// Function

// fn main() {
//     define_x();
// }

// fn define_x() {
//     let x: &str = "hello";

//     println!("{}, world", x)
// }

// =======================================================
// Shadowing

// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x); // 42
// }

fn main() {
    let mut x: i32 = 1;
    x = 7; // 7

    // shadowing and re-binding
    // let mut x = x; // 7 => useless
    x += 3;

    println!("{}", x); // 10

    let y: i32 = 4;
    // shadowing
    let y: &str = "I can also be bound to text";

    println!("Success")
}
