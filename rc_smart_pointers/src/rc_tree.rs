use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>, 
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node <T>
{
    fn new_root(value: T) -> Rc<Node<T>> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    fn new_leaf(value: T, parent: &mut Rc<Node<T>>) -> Rc<Node<T>> {
        let result = Rc::new( Node {
            value,
            parent: RefCell::new(Rc::downgrade(&parent)),
            children: RefCell::new(vec![]),
        });

        if let Some(node) = Rc::get_mut(parent) {
            node.append_child(&result);
        }

        result
    }
    
    fn append_child(&mut self, child: &Rc<Node<T>>) {
        self.children.borrow_mut().push(Rc::clone(child));
    }
}

pub fn create_rc_tree() {
    let mut n = Node::new_root(10);
    let mut child = Node::new_leaf(10, &mut n);
}
