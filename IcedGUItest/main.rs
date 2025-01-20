// Login Form

use iced::{widget::{self, button, column, container, text, text_input}, Element};

pub fn main() -> iced::Result {
    iced::run("Login Form", update, view)
}

#[derive(Default)]
struct State {
    user: String,
    password: String,
}

#[derive(Debug, Clone)]
enum Message {
    SendForm,
    ShowPassword,
    HidePassword,
    UserChanged(String),
    PasswordChanged(String),
    Print(String),
}

#[derive(Default)]
struct User {
    name: String,
    password: String,
    account_type: String,
}

fn setup_users() -> Vec<User>{
    vec![
        User {
            name: String::from("Leon"),
            password: String::from("12345"),
            account_type: String::from("Child"),
        },
        User {
            name: String::from("Max"),
            password: String::from("secure_password"),
            account_type: String::from("Parent"),
        }
    ]
}

fn view(state: &State) -> Element<'_, Message> {
    column![
        text_input("Name", &state.user)
        .on_input(Message::UserChanged),

        text_input("Password", &state.password)
        .on_input(Message::PasswordChanged),

        button("Send")
        .on_press(Message::SendForm),
    ]
    .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::UserChanged(content) => {
            state.user = content;
        },
        Message::PasswordChanged(content) => {
            state.password = content;
        },
        Message::SendForm => {
            let users = setup_users();
            let valid_user = users.iter().find(|user| {
                user.name == state.user && user.password == state.password
            });
            match valid_user {
                Some(user) => {
                    println!("Logged in as {}", user.name)
                }
                None => {
                    println!("Invalid username or password")
                }
            }
        },
        Message::Print(content) => {
            println!("Print: {}", content)
        },
        _ => {
            println!("undhandled")
        }
    }
}
