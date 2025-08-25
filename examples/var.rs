#![allow(unused)]

// Constants
const NUM: u32 = 1;
// Variables live inside the memory and the constants can live inside the compiled code
fn main(){
    let mut x = 1;

    // Type Inference
    let y : i32 = -1;
    let z = -1;

    // Shadowing - we can redeclare a variable with different value or type
    let a: i32 = -1;
    let a: i32 = -2;
    let a: bool = true;

    // Type placeholder
    let b: _ = 1234;

    // println!
    let x: _ = 2;
    println!("the value of x is {}", x);
    // Inline

    println!("the value of x is {x}");
    // Positional Arguments
    println!("{0} + {0} = {1}", x, x + x);

    //Debug
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x);
}