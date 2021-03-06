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

        get_reference(&s3);
        
        // does not compile because s4 is not mutable and previously was moved
        // takes_ownership_mutate(&s4);
        
        let mut s4 = s3.clone();
        // now it works
        takes_ownership_mutate(&mut s4);
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

fn takes_ownership_mutate(string: &mut String) -> usize {
    string.push_str("appended");
    string.len()
}

fn get_reference(string: &String) -> usize {
    return string.len();
}
