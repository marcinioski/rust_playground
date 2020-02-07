enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

mod refcel_ex;
mod combine_rc_refcell;
mod reference_cycle;
mod rc_tree;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("a rc: {}", Rc::strong_count(&a));
    // borrowed reference not moved!
    let b = Cons(3, Rc::clone(&a));
    println!("a rc: {}", Rc::strong_count(&a));
    // so it is posisble:
    let c = Cons(4, Rc::clone(&a));
    println!("a rc: {}", Rc::strong_count(&a));

    combine_rc_refcell::call1();
    reference_cycle::create_reference_cycle();
    rc_tree::create_rc_tree();
}
