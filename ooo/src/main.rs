use std::rc::Rc;

pub trait Call {
    fn call(self: Box<Self>) -> Box<dyn Call>;
}

struct Base {
    pub caller: Option<Box<dyn Call>>,
}

struct DeriveA {
}

impl Call for DeriveA {
    fn call(self: Box<Self>) -> Box<dyn Call> {
        println!("DeriveA call");
        Box::new(DeriveA {})
    }
}

impl Base {
    fn new() -> Base {
        Base {
            caller: Some(Box::new(DeriveA {} )),
        }
    }

    fn borrow(&self) -> Result<& Box<dyn Call>, &'static str> {
/*        match self.caller {
            Some(ref x) => Ok(&x),
            None => Err("not found"),
        }
*/
        self.caller.as_ref().ok_or("error")
    }
}

fn main() {
    println!("Hello, world!");
    let mut b = Base::new();
//    *b.borrow().unwrap().call();

    if let Some(caller) = b.caller.take() {
        caller.call();
    }
}
