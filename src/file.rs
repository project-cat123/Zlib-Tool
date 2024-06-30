use iced::{Element, Length, widget::{Column, Text, TextInput, Button}, theme};
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use rfd::FileDialog;
use crate::message::FileMessage;
use crate::style::{CustomButtonStyle, CustomTextInputStyle};

#[derive(Default)]
pub struct FileTab {
    pub input_path: String,
    pub file_content: String,
    pub output_path: String,
    pub result: String,
    pub request_headers: String,
}

impl FileTab {
    pub fn update(&mut self, message: FileMessage) {
        match message {
            FileMessage::InputPathChanged(path) => {
                self.input_path = path;
            }
            FileMessage::FileContentChanged(content) => {
                self.file_content = content;
            }
            FileMessage::OutputPathChanged(path) => {
                self.output_path = path;
            }
            FileMessage::LoadFile => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.input_path = path.to_string_lossy().to_string();
                    match read_binary_file(&self.input_path) {
                        Ok(content) => {
                            let (headers, body) = split_headers_and_body(&content);
                            self.request_headers = headers;
                            self.result = match decompress_binary(&body) {
                                Ok(decompressed) => decompressed,
                                Err(err) => format!("Decompression error: {}", err),
                            };
                            self.file_content = self.result.clone();
                        }
                        Err(err) => self.result = format!("Load error: {}", err),
                    }
                }
            }
            FileMessage::SaveFile => {
                if let Some(path) = FileDialog::new().save_file() {
                    self.output_path = path.to_string_lossy().to_string();
                    match compress_binary(&self.file_content) {
                        Ok(compressed_body) => {
                            let data_to_save = [&self.request_headers.as_bytes()[..], &compressed_body[..]].concat();
                            if let Err(err) = write_binary_file(&self.output_path, &data_to_save) {
                                self.result = format!("Save error: {}", err);
                            } else {
                                self.result = "File saved successfully".to_string();
                            }
                        }
                        Err(err) => self.result = format!("Compression error: {}", err),
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<FileMessage> {
        Column::new()
            .push(
                TextInput::new("Input file path", &self.input_path)
                    .on_input(FileMessage::InputPathChanged)
                    .padding(10)
                    .style(theme::TextInput::Custom(Box::new(CustomTextInputStyle)))
                    .width(Length::Fill),
            )
            .push(
                TextInput::new("File content", &self.file_content)
                    .on_input(FileMessage::FileContentChanged)
                    .padding(10)
                    .style(theme::TextInput::Custom(Box::new(CustomTextInputStyle)))
                    .width(Length::Fill),
            )
            .push(
                TextInput::new("Output file path", &self.output_path)
                    .on_input(FileMessage::OutputPathChanged)
                    .padding(10)
                    .style(theme::TextInput::Custom(Box::new(CustomTextInputStyle)))
                    .width(Length::Fill),
            )
            .push(
                Button::new(Text::new("Load File"))
                    .on_press(FileMessage::LoadFile)
                    .style(theme::Button::Custom(Box::new(CustomButtonStyle)))
                    .padding(10)
                    .width(Length::Fill),
            )
            .push(
                Button::new(Text::new("Save File"))
                    .on_press(FileMessage::SaveFile)
                    .style(theme::Button::Custom(Box::new(CustomButtonStyle)))
                    .padding(10)
                    .width(Length::Fill),
            )
            .push(Text::new(&self.result))
            .into()
    }
}

fn compress_binary(input: &str) -> Result<Vec<u8>, String> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(input.as_bytes()).map_err(|e| e.to_string())?;
    e.finish().map_err(|e| e.to_string())
}

fn decompress_binary(input: &[u8]) -> Result<String, String> {
    let mut d = ZlibDecoder::new(input);
    let mut s = Vec::new();
    d.read_to_end(&mut s).map_err(|e| e.to_string())?;
    String::from_utf8(s).map_err(|e| e.to_string())
}

fn read_binary_file(path: &str) -> Result<Vec<u8>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents).map_err(|e| e.to_string())?;
    Ok(contents)
}

fn write_binary_file(path: &str, data: &[u8]) -> Result<(), String> {
    let file = File::create(path).map_err(|e| e.to_string())?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(data).map_err(|e| e.to_string())
}

fn split_headers_and_body(content: &[u8]) -> (String, Vec<u8>) {
    if let Some(pos) = content.windows(2).position(|window| window == b"\x78\x9c") {
        let headers = &content[..pos];
        let body = &content[pos..];
        let headers_str = String::from_utf8_lossy(headers).to_string();
        (headers_str, body.to_vec())
    } else {
        (String::new(), content.to_vec())
    }
}
