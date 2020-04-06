trait Base<T> {
    fn base(&mut self) {
        println!("Call from base");
    }
}

trait Derived: Base<Derived> {
    fn derived(&mut self) {
        println!("Call from derived");
    }
}

struct MyDerived {}

impl Derived for MyDerived {
}

#[cfg(test)]
mod crtp_test {
    use super::*;

    #[test]
    fn derive_test() {
        let mut md = MyDerived {};
    }
}
