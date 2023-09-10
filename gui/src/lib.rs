pub use iced;

use iced::{
    widget::{column, text},
    Element, Sandbox,
};

pub struct App;

impl Sandbox for App {
    type Message = ();

    fn new() -> Self {
        App
    }

    fn title(&self) -> String {
        String::from("GitGui")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        column![text("Hello")].into()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
