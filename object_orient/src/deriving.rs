struct A {}
struct B {}
struct C {}

struct Aggregator<S>{
    _val: S,
}

impl<S> Aggregator<S> {
    pub fn new(s: S) -> Aggregator<S> {
        Aggregator{_val: s}
    }
}

#[cfg(test)]
mod deriving_tests {
    use super::*;
    #[test]
    fn deriving_test () {
        let a = Aggregator::new(A {} );
        let b = Aggregator::new(B {} );


    }

}
