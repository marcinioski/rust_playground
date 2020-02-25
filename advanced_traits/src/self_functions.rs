trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human { }

impl Pilot for Human {
    fn fly(&self) {
        println!("Getting plane");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Magically goes up");
    }
}

impl Human {
    fn fly(&self) {
        println!("Are you kidding me?");
    }
}

pub fn call1() {
    let h = Human{};
    Pilot::fly(&h);
    Wizard::fly(&h);
    h.fly();
}
