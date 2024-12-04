pub mod screen1;
pub mod screen2;

use iced::{widget::horizontal_space, Element};
use screen1::Screen1;
use screen2::Screen2;

pub fn loading<'a, Message: 'a>() -> Element<'a, Message> {
    horizontal_space().into()
}

pub enum Screen {
    Loading,
    Screen1(Screen1),
    Screen2(Screen2),
}
