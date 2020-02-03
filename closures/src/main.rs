struct Cacher<T>
    where T: Fn(u32) -> u32
{
    call1: T,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            call1: calculation,
        }
    }

    fn call(&self, x: u32) -> u32 {
        (self.call1)(x)
    }
}
/*
struct Cacher2<T, R>
        where T: Fn(R) -> R
{
    call2: T,
}

impl<T, R> Cacher<T, R>
    where T: Fn(R) -> R
{
}
*/

fn main() {
    println!("Hello, world!");

    let c = Cacher::new(|x: u32|{
        x
    });

    println!("{}", c.call(10));
}
