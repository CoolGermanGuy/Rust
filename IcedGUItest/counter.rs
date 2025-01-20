// Counter
use iced::widget::{button, column, text};
use iced::Element;

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn view(counter: &Counter) -> Element<Message> {
    column![
        button("+").on_press(Message::Increment),
        text(counter.value).size(20),
        button("-").on_press(Message::Decrement)
    ]
    .spacing(10)
    .into()
}
