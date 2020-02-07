use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use crate::reference_cycle::List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }

    fn get_val(&self) -> Option<i32> {
        match self {
            Cons(value, _) => Some(*value),
            Nil => None,
        }
    }
}

pub fn create_reference_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count {}", Rc::strong_count(&a));
    println!("b rc count {}", Rc::strong_count(&b));
    println!("b next item {:?}", b.tail());

    // create reference cycle:
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    if let Some(val) = a.get_val() {
        println!("a get_val: {}", val);
    }

    if let Some(b) = a.tail() {
        let b = &*b;
        if let Some(b) = b.borrow_mut().get_val( ){
            println!("b : {}", b); 
        }
    }
}
