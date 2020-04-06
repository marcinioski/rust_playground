trait Animal {
    fn eat(&mut self) {
        println!("Animal is eating");
    }
}

trait Cat: Animal {
    fn meow(&mut self) {
        println!("Meow!...");
    }
}

struct MyCat;

impl Animal for MyCat {
    fn eat(&mut self) {
        println!("Cat is eating");
    }
}

impl Cat for MyCat {
}

fn animal_eat(animal: &mut dyn Animal) {
    animal.eat();
}

mod other_trait;
//mod crtp;
mod deriving;

fn main() {
/*
 * let mut cat = MyCat {};
    cat.eat();
    Animal::eat(&mut cat);
    cat.meow();

    animal_eat(&mut cat);
*/
}
