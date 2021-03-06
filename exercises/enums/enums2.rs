// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!


// found solution:
// https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{ x: i32, y: i32 },
    Quit,
    Echo(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
