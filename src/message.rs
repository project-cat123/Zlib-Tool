#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Home,
    File,
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchTab(Tab),
    HomeMessage(crate::home::HomeMessage),
    FileMessage(crate::message::FileMessage),
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    InputPathChanged(String),
    FileContentChanged(String),
    OutputPathChanged(String),
    LoadFile,
    SaveFile,
}
