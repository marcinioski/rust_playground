trait Animal<T> {
    fn eat(&mut self) {
        println!("Animal eats");
    }
}

trait Cat<T>: Animal<T> {
    fn meow(&self) {
        println!("Cat: meow");
    }
}

struct MyCat {}

impl Animal<MyCat> for MyCat{
}

#[cfg(test)]
mod trait_test {
    use super::*;
    #[test]
    fn call_test() {
        let mut cat = MyCat {};
        cat.eat();
    }
}
