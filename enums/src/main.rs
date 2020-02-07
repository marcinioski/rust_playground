enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    addr: String,
    kind: IpAddrKind,
}

enum IPAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move(u8, u8),
    Information(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Information(value) => println!("{}", value),
            _ => (),
        }
    }
}

fn route (ip: IpAddrKind) {

}

fn main() {
    println!("Hello, world!");

    let ipv_1 = IpAddrKind::V4;

    let home = IpAddr {
        addr: String::from("localhost"),
        kind: ipv_1,
    };

    route(home.kind);

    let home = IPAddr::V4(home.addr);
}
