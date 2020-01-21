mod string_utf;
mod fun_with_hash;

fn create_vector() -> Vec<i32>{
    let mut v: Vec<i32> = Vec::new();
    
    v.push(5);
    v.push(6);

    v 
//    let v = vec![1 ,2 ,3];
}

enum OwnEnum {
    Empty,
    StringVal(String),
    IntVal(i32),
}

fn strange_vector() {
    let mut v: Vec<OwnEnum> = Vec::new();
    v.push(OwnEnum::StringVal(String::from("empty")));
}

fn read_vector(v: &Vec<i32>) {
    let zero: &i32 = &v[0];

    match v.get(5) {
        Some(zero) => println!("The zeroth element is {}", zero),
        None => println!("None"),
    }

    for i in v {
        println!("i'th element {}", i);
    }
}

fn main() {
    println!("Hello, world!");

    read_vector(&create_vector());
    strange_vector();
    string_utf::first_call();
    fun_with_hash::fun_map();

    let mut x: i32 = 100;

    let y = &x;
}
