use iced::{Element, Length, widget::{Column, Text, TextInput, Button, Row, Scrollable}, theme};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use crate::style::{CustomButtonStyle, CustomTextInputStyle};

#[derive(Default)]
pub struct Home {
    pub input_text: String,
    pub result: String,
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InputChanged(String),
    Decompress,
    Compress,
}

impl Home {
    pub fn update(&mut self, message: HomeMessage) {
        match message {
            HomeMessage::InputChanged(text) => {
                self.input_text = text;
            }
            HomeMessage::Decompress => {
                self.result = decompress_text(&self.input_text);
                self.input_text = self.result.clone(); // 압축 해제 결과를 다시 입력창에 반영
            }
            HomeMessage::Compress => {
                self.result = compress_text(&self.input_text);
                self.input_text = self.result.clone(); // 압축 결과를 다시 입력창에 반영
            }
        }
    }

    pub fn view(&self) -> Element<HomeMessage> {
        Column::new()
            .push(
                TextInput::new("Input text", &self.input_text)
                    .on_input(HomeMessage::InputChanged)
                    .padding(10)
                    .width(Length::Fill)
                    .style(theme::TextInput::Custom(Box::new(CustomTextInputStyle))),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(Text::new("Decompress"))
                            .on_press(HomeMessage::Decompress)
                            .padding(10)
                            .width(Length::Fill)
                            .style(theme::Button::Custom(Box::new(CustomButtonStyle))),
                    )
                    .push(
                        Button::new(Text::new("Compress"))
                            .on_press(HomeMessage::Compress)
                            .padding(10)
                            .width(Length::Fill)
                            .style(theme::Button::Custom(Box::new(CustomButtonStyle))),
                    ),
            )
            .push(
                Scrollable::new(
                    Text::new(&self.result)
                        .width(Length::Fill)
                        .size(16)
                )
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .into()
    }
}

fn decompress_text(input: &str) -> String {
    let bytes = hex::decode(input.replace(" ", "")).unwrap_or_default();
    let mut d = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap_or_default();
    s
}

fn compress_text(input: &str) -> String {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(input.as_bytes()).unwrap_or_default();
    let compressed_bytes = e.finish().unwrap_or_default();
    hex::encode(compressed_bytes)
}
