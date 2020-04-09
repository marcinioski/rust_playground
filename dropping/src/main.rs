struct X<'a>(&'a str);

impl<'a> Drop for X<'a> {
    fn drop(&mut self) {
        println!("Dropping X {}", self.0);
    }
}

fn main() {
    let mut x = Box::new(X("1"));
    let y = &mut x;
    *y = Box::new(X("2"));

    let x: Box<X>;

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        x = Box::new(X("3"));
    }
    // drop of uninitialized causing compilation error:
    // drop(&x);
}
