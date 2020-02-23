enum Message {
    Hello {id: i32},
}

fn main() {
    println!("Hello, world!");

    let _new = 1;

    let numbers = (2, 3, 4, 5);

    match numbers {
        (_, second, ..) => {
            println!("Second value: {}", second);
        }
    }

    let msg = Message::Hello{ id: 10 };

    match msg {
        Message::Hello {id: id_variable @ 9..=11 } => {
            println!("id_variable: {}", id_variable);
        },
        Message::Hello { id: _ } => {
            println!("Other value");
        },
    }
}
