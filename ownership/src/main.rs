fn _internal_function() -> String{
    let sample = "_internal";
    let mut s = String::from(sample);

    s.push_str("_function");

    return s;
}

fn main() {
    println!("{}", _internal_function());

    let mut s = String::from("sample string");

    let s2 = s;
    
    // s is moved so it cannot be moved again
    // let s3 = s;
    //
    
    {
        let s3 = s2;

        // only way to fix it is clone it:
        let s4 = s3.clone(); 

        println!("{}", takes_ownership_and_return_copy(s4));
    }
    // s2 was moved in scope above!
    //println!("ex: {}", s2);
}

fn takes_ownership_and_return_copy(string: String) -> String {
    string
}

fn takes_ownership_and_return_copy_and_size(string: String) -> (usize, String) {
    (string.len(), string)
}
