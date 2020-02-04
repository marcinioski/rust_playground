use std::ops::Deref;
use std::mem::drop;

mod custom_smart_pointer;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_list() -> crate::List {
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));

    list
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn print_string(val: &str) {
    println!("{}", val);
}

#[test]
fn test_my_box() {
    let x = MyBox::new(10);

    assert_eq!(10, *x);
    assert_eq!(10, *(x.deref()));
    assert_eq!(10, *(&x.0));
}

#[test]
fn test_my_box_string() {
    let x = MyBox::new(String::from("Hello world!"));

    print_string(&x);
}

fn main() {
    let b = Box::new(5);

    let x = create_list();

    drop(x);

    custom_smart_pointer::test_dropping();
}
