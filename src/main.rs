use iced::widget::{button, column, text, Button, Column, Row, Text};
use iced::Center;
use iced::Element;

pub fn main() -> iced::Result {
    iced::run("A ctf tools", Counter::update, Counter::view)
}

#[derive(Debug, Clone)]
pub enum AppState {
    MainMenu,
    Window1,
    Window2,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState::MainMenu
    }
}

#[derive(Default)]
struct Counter {
    value: i64,
    state: AppState,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    GoToWindow1,
    GoToWindow2,
    BackToMenu,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::GoToWindow1 => {
                self.state = AppState::Window1;
            }
            _ => (),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::MainMenu => {
                let button1 = Button::new("Increment").on_press(Message::Increment);
                let text = Text::new(self.value).size(50);
                let button2 = Button::new("Decrement");
                let input_row = Row::new()
                    .spacing(20)
                    .push(button1)
                    .push(text)
                    .push(button2)
                    .align_y(Center);

                Column::new()
                    .padding(20)
                    .push(Text::new("Please enter values in both fields:"))
                    .push(input_row)
                    .into()
            }
            _ => {
                let button1 = Button::new("Increment");
                let text = Text::new(self.value).size(50);
                let button2 = Button::new("Decrement");
                let input_row = Row::new()
                    .spacing(20)
                    .push(button1)
                    .push(text)
                    .push(button2)
                    .align_y(Center);

                Column::new()
                    .padding(20)
                    .push(Text::new("Please enter values in both fields:"))
                    .push(input_row)
                    .into()
            }
        }
    }
}
