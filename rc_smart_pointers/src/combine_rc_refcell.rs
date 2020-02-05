use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List2 {
    Cons(Rc<RefCell<i32>>, Rc<List2>),
    Nil,
}

use crate::combine_rc_refcell::List2::{Cons, Nil};

pub fn call1() {
    let value = Rc::new(RefCell::new(10));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("c after: {:?}", c);
}
