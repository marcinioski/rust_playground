fn main() {
    println!("Hello, world!");

    let y = another_function(1200);

    println!("return from another function: {}", y);

    let result = if y < 100 {
        "y less than 100"
    } else if y == 200 {
        "y is equal 200"
    } else {
        "else"
    };

    println!("result: {}", result);
}

fn another_function(x: i32) -> i32 {
    println!("another function: {}", x);   

    let x = {
        x*2
    };

    println!("another function {}", x);

    x
}

//fn another_function() {
//    println!("Hello from another function");
//}
