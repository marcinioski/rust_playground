pub fn first_call() {
    let data = "initial contents";

    let s = data.to_string();

    let s = format!("{}-{}", data, s);
    println!("{}", s);
}
