use hellomacro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

/*
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! Pancakes");
    }
}
*/

fn main() {
    Pancakes::hello_macro();
}
