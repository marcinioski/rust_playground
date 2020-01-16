struct User {
    username: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    let s = String::from("one two");
    println!("first word len: {}", first_word(&s));
    println!("first word: {}", string_slices(&s));
    println!("second word: {}", second_word(&s));
}

fn first_word(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    string.len()
}

fn second_word(string: &String) ->  &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[i+1..string.len()];
        }
    }

    &string[..]
}

fn string_slices(string: &String) -> &str{
    let first = first_word(&string);
    // s is equal to s2
    let s = &string[0..first];
    let s2 = &string[..first];

    s2
}

fn struct_playground() {
    let user1 = User {
        username: String::from("johnny_doe"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("johnny_doe"));
    let user2 = rebuild_user3(user2);
}

fn build_user(username: String) -> User {
    User {
        username: username,
        sign_in_count: 1,
    }
}

fn build_user2(username: String) -> User {
    User {
        sign_in_count: 1,
        username,
    }
}

fn rebuild_user3(user: User) -> User {
    User {
        ..user
    }
}
