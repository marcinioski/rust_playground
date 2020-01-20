#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn call_outside_a() {}

mod sample_module_namespace_1 {
    struct Module1Struct {
        pub value: u32,
    }

    pub mod module_a {
        pub fn call_a() {
            crate::sample_module_namespace_1::module_b::call_d();
            call_b();
        }
        fn call_b() {
            super::super::call_outside_a();
        }
    }

    fn call_outside_a() {
    }

    mod module_b {
        fn call_a() {}
        pub fn call_d() {}
        fn call_e() {}
    }
}

pub fn call_function_chain() {
    sample_module_namespace_1::module_a::call_a();
    crate::sample_module_namespace_1::module_a::call_a();
}

use crate::sample_module_namespace_1::module_a;
mod second_module;
use second_module::module_aa;

fn internal_call() {
    module_aa::call_aa();
}
