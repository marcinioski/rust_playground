type Kilometers = i32;

type thunk = Box<dyn Fn() + Send + 'static>;

type ResultT<T> = std::result::Result<T, &'static str>;

fn main() {
    println!("Hello, world!");

    let x: i32 = 40;
    let y: Kilometers = 5;

    let z = x + y;

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: thunk = Box::new(|| println!("hi"));

    let r: ResultT<thunk> = Ok(f);
}
