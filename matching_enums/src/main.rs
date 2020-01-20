enum EMessage {
    Wrong,
    ErrorValue (u32),
    Message(String)
}

impl EMessage {
    fn show(&self) -> Option<u32> {
        match self {
            EMessage::Wrong => { println!("Wrong value"); None },
            EMessage::ErrorValue(value) => { println!("Got error value: {}", value); Some(1)},
            EMessage::Message(message) => { println!("Got message: \"{}\"", message); None} ,
            _ => None,
        }
    }
}

fn iteratate_through_emessage(emessages: &[EMessage]) {
    for element in emessages.iter().enumerate() {
        element.1.show();
    }
}

fn main() {
    println!("Hello, world!");
    let em1 = EMessage::Wrong;
    let mut em2 = EMessage::ErrorValue(10);
    
    em1.show();
    em2.show();

    em2 = EMessage::Message(String::from("hello"));

    em2.show();

    if let EMessage::Wrong = em2 {
        ()
    }
    else {
        println!("Rust is awesome");
    }

    let em_slices: [EMessage; 3] = [EMessage::Wrong, EMessage::ErrorValue(999), EMessage::Message(String::from("bye"))];

    iteratate_through_emessage(&em_slices);
}
