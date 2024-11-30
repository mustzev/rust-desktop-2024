use iced::{
    widget::{button, text},
    Element,
};

fn main() -> iced::Result {
    iced::run("Rust Desktop 2024", update, view)
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u32,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    button(text(counter.value))
        .on_press(Message::Increment)
        .into()
}
