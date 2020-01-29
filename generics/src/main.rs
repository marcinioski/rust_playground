/*
fn largest<T>(list: &[T]) -> T {
    let mut result: T = list[0];

    for &ii in list.iter() {
        if ii > result {
            result = ii;
        }
    }

    result
}
*/

mod own_trait;
mod lifetimes;

use own_trait::OwnTrait;

struct Point<T> {
    x: T,
    y: T,
}

enum DiffType<T> {
    Ok(T),
    None,
}

impl<T> Point<T> {
    fn isOk(&self) -> DiffType<i32> { 
        return DiffType::Ok(20)
    }
}

impl<T> OwnTrait for Point<T> {
    fn call1(&self) -> String {
        //let mut result = format!("x: {} y: {}", self.x, self.y);

        let result = format!("not ok");

        String::from(result)
    }
}

pub fn call1(obj: impl OwnTrait) {
    obj.call1();
}

pub fn call2<T: OwnTrait>(obj: T) {
    obj.call2();
}

pub fn callWhere<T>(obj: &T)
    where T: OwnTrait {
        obj.call1();
}

fn main() {
    println!("Hello, world!");

    let list1 = vec![10, 20, 3, 4, 100];

 //   println!("largest: {}", largest(&list1));

    let p1 = Point{x: 1, y: 2};

    println!("{}", p1.call1());
    println!("{}", p1.call2());

    callWhere(&p1);
    call2(p1);
//    call1(p1);
    let x = lifetimes::longest("long", "longest");

    let mut result: &str;
    {
        let y = "much longer";
        result = lifetimes::longest(y, x);
    }

    println!("{}", result);
}
