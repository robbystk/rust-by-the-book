enum Message {
    Quit,
    Move { x: f32, y: f32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // pass
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));

    m.call();

}
