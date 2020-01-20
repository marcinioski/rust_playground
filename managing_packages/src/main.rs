use sample_module;

use sample_module::sample_module_namespace_1 as smn1;

fn main() {
    println!("Hello, world!");
    sample_module::internal_call();
    smn1::module_a::call_a();
    sample_module::call_function_chain();
}
