/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 *
 * To run this exercise you can either use the build in tools of VSCode or start via `cargo run` in this directory.
 */
fn main() {
    // Introduce another variable here to make the print statement work

    let mut x = 10;
    let y: i32 = 20;
    println!("Hello, world! X: {}, Y: {}", x, y);

    // Modify x here, give it a new value. Rust offers two methods to do this, can you find them both?
    x = 15;
    println!("Hello, world! X: {}, Y: {}", x, y);

    let x = 17;
    println!("Hello, world! X: {}, Y: {}", x, y);

    // Swap x and y here in one action
    let (x, y) = (y,x);
    println!("Hello, world! X: {}, Y: {}", x, y);
}
