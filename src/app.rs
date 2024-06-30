use iced::{
    executor, widget::{Button, Column, Container, Row, Text},
    Application, Command, Element, Length, theme,
};
use crate::message::{Message, Tab};
use crate::home::Home;
use crate::file::FileTab;
use crate::style::{CustomButtonStyle, CustomContainerStyle, CustomBackgroundStyle}; // Import styles

pub struct App {
    current_tab: Tab,
    home: Home,
    file: FileTab,
}

impl Default for App {
    fn default() -> Self {
        App {
            current_tab: Tab::Home,
            home: Home::default(),
            file: FileTab::default(),
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Zlib-Tool")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::SwitchTab(tab) => {
                self.current_tab = tab;
            }
            Message::HomeMessage(msg) => {
                self.home.update(msg);
            }
            Message::FileMessage(msg) => {
                self.file.update(msg);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let tab_view: Element<_> = match self.current_tab {
            Tab::Home => self.home.view().map(Message::HomeMessage),
            Tab::File => self.file.view().map(Message::FileMessage),
        };

        let menu = Column::new()
            .push(
                Button::new(
                    Text::new("Home")
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                )
                .width(Length::Fill)
                .style(theme::Button::Custom(Box::new(CustomButtonStyle)))
                .on_press(Message::SwitchTab(Tab::Home))
            )
            .push(
                Button::new(
                    Text::new("File")
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                )
                .width(Length::Fill)
                .style(theme::Button::Custom(Box::new(CustomButtonStyle)))
                .on_press(Message::SwitchTab(Tab::File))
            );

        Container::new(
            Row::new()
                .push(Container::new(menu).width(Length::Fixed(150.0)).style(theme::Container::Custom(Box::new(CustomContainerStyle))))
                .push(Container::new(tab_view).style(theme::Container::Custom(Box::new(CustomBackgroundStyle))).width(Length::Fill).height(Length::Fill))
        )
        .style(theme::Container::Custom(Box::new(CustomContainerStyle))) // 전체 배경에 스타일 적용
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

