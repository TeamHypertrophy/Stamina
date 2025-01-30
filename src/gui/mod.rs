use iced::{Application, Command, Settings};

#[derive(Default, Clone)]
pub struct TestosteroneGui ;


impl Application for TestosteroneGui {
    type Executor = iced::executor::Default;
    type Message = ();
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Testosterone")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello, Testosterone!".into()
    }
}

impl TestosteroneGui {
    pub fn start() -> iced::Result {
        Self::run(Settings::default())
    }
}