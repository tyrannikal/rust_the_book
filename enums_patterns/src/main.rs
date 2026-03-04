#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {x}, y: {y}"),
            Message::Write(a_string) => println!("Write: {a_string}"),
            Message::ChangeColor(x, y, z) => println!("ChangeColor to x: {x}, y: {y}, z: {z}"),
        }
    }
}

fn main() {
    let home = dbg!(IpAddr::V4(String::from("127.0.0.1")));
    let loopback = dbg!(IpAddr::V6(String::from("::1")));

    match &home {
        IpAddr::V4(addr) => println!("IP4 address: {addr}"),
        IpAddr::V6(addr) => println!("IP6 address: {addr}"),
    }

    match &loopback {
        IpAddr::V4(addr) => println!("IP4 address: {addr}"),
        IpAddr::V6(addr) => println!("IP6 address: {addr}"),
    }

    let m1 = Message::Write(String::from("hello"));
    m1.call();

    let m2 = Message::Move { x: 2, y: 4 };
    m2.call();

    let m3 = Message::ChangeColor(2, 4, 8);
    m3.call();

    let m4 = Message::Quit;
    m4.call();
}
