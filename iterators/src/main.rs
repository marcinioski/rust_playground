mod filter_example;

fn main() {
    let v1 = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    filter_example::call1();
}
