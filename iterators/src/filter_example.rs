#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 12, style: String::from("one")},
        Shoe { size: 12, style: String::from("one")},
        Shoe { size: 13, style: String::from("one")},
        Shoe { size: 145, style: String::from("two")},
    ];

    let in_my_size = shoes_in_my_size(shoes, 12);
}

pub fn call1() {
}
