use std::collections::HashMap;

pub fn fun_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    f1();
    f2();
}

fn f1() {
    let teams = vec!["blue".to_string(), "red".to_string()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = "blue".to_string();
    let score = scores.get(&team_name);
}

fn f2() {
    let text = "hello world hello";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
