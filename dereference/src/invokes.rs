struct X (i32);

trait Invoker {
    fn invoke(self);
}

impl Invoker for X {
    fn invoke(self) {
        println!("self: val {}", self.0);
    }
}

impl Invoker for &X {
    fn invoke(self) {
        println!("&self val: {}", self.0);
    }
}

impl Invoker for &&X {
    fn invoke(self) {
        println!("&&self val: {}", self.0);
    }
}

impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl Invoker for *const X {
    fn invoke(self) {
        println!("*const self val: ");
    }
}

struct Y(i32);

impl Invoker for Y {
    fn invoke(self) {
        println!("self: val: {}", self.0);
    }
}

impl std::ops::Deref for Y {
    type Target = Y;

    fn deref(&self) -> &Y {
        &self
    }
}

pub fn invoke() {
    let x = X(0i32);

    (&&x).invoke();
    (&x).invoke();
    (&&&&&&&x).invoke();
    x.invoke();

    let x = X(90i32);
    let rx = &x;
    let px: *const X = &x;
    px.invoke();
    //(*x).invoke();
    println!("val: {}", *x);

    // reached the recursion limit
    //(*Y(10)).invoke();
}
