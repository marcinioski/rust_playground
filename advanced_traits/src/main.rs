use std::ops::Add;

trait SampleTrait {
    type Item;
    fn call1(&mut self) -> Option<Self::Item>;
}

struct SampleStruct {
    x: i32,
}

impl SampleTrait for SampleStruct {
    type Item = i32;
    fn call1(&mut self) -> Option<Self::Item> {
        return Some(self.x);
    }
}

// operator overloading
impl Add for SampleStruct {
    type Output = SampleStruct;

    fn add(self, other: SampleStruct) -> SampleStruct {
        SampleStruct {x: self.x + other.x}
    }
}

trait OwnTrait<RHS=Self>{
    type Output;
    fn call2(self, rhs: RHS) -> Self::Output;
}

struct OwnStruct {
    x: i32,
}

impl OwnTrait for OwnStruct {
    type Output = i32;
    fn call2(self, rhs: OwnStruct) -> Self::Output {
        self.x + rhs.x
    }
}

impl OwnTrait<SampleStruct> for OwnStruct {
    type Output = i32;

    fn call2(self, rhs: SampleStruct) -> i32 {
        self.x + rhs.x
    }
}

mod self_functions;

fn main() {
    println!("Hello, world!");
    self_functions::call1();
}
