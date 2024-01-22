#![windows_subsystem = "windows"]
use gui::{
    iced::{self, Application, Settings},
    App,
};

fn main() -> iced::Result {
    App::run(Settings::default())
}
