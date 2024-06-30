mod app;
mod home;
mod file;
mod style;
mod message;

use iced::{Application, Settings};
use app::App;

fn main() -> iced::Result {
    App::run(Settings::default())
}
