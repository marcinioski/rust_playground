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

    println!("loop_function {}", loop_function(10));

    while_loop(1);

    interating_in_for();

    interating_in_for_range();
}

fn loop_function(x: u32) -> u32 {

    /*
     * never ending loop
     */
    
    //loop {
    //    println!("inside loop");
    //}
    //
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == x {
            break counter;
        }
    };

    result
}

fn while_loop(x: u32) {
    let mut counter = 0;

    let result = while counter != x {
        counter += 1;
    };
}

fn interating_in_for() {
    let a = [5, 10, 15];

    for element in a.iter() {
        println!("the value is {}", element);
    }
}

fn interating_in_for_range() {
    let a = [5, 10, 15, 20];

    // NOT a.len() -1 !!!
    for number in (0..a.len()) {
        println!("value: {}", a[number]); 
    }
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
