pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }
    y
}

pub struct StructResult<'a> {
    x: &'a str,
}

impl<'a> StructResult<'a> {
    pub fn call1(&self) -> &'a str {
        return self.x;
    }
}

pub enum FunctionResult<T, U> {
    Ok(T),
    NotOk(U),
}

struct AnotherStruct<'a, T> {
    x: &'a T,
}

impl <'a, T> AnotherStruct<'a, T> {
    fn anotherCall(&self, input: &'a T) -> AnotherStruct<T> {
        return AnotherStruct{x: input}
    }
}
