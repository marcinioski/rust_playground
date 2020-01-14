const MAX_POINTS: u32 = 10_000;

fn main() {
    println!("Hello, world!");

    let mut x = 5;

    println!("Value of x is {}", x);

    x = 6;

    println!("Value of x is {}", x);

    x = x + MAX_POINTS;

    println!("Value of x is {}", x);

    /*************************************************************************/
    /* shadowing */
    /*************************************************************************/

    let x = 3;

    println!("Value of x is {}", x);

    // x = 4; // causing error, x is not mutable
 
    // let spaces = "     "; // unused, causing warning
    // let spaces = spaces.len(); // unused, causing warning

    // let mut spaces = "    "; // unused, causing warning
    // spaces = spaces.len(); // does not compile cause by type
    // spaces = spaces.len().to_string(); // does not compile cause by type
    
    let x = 3.0; // f64 default compiler value
    //let x: f32 = 2.0; // f32

    println!("Value of x is {}", x);

    /*************************************************************************/
    /* tuple type */
    /*************************************************************************/
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x {} y {} z {}", x, y, z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

 //   let five_hundred = tup.0; // unused variable
 //   let six_point_four = tup.1; // unused variable
 
    /*************************************************************************/
    /* array type */
    /*************************************************************************/

    let months = ["Jan", "Feb", "Mar"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
}   
