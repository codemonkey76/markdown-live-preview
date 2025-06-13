use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum Message {
    Init(InitMessage),
    BufferChange(BufferChangeMessage),
    CursorMoved(CursorMessage),
}

#[derive(Debug, Deserialize)]
pub struct CursorMessage {
    pub cursor: (usize, usize),
}

#[derive(Debug, Deserialize)]
pub struct InitMessage {
    pub content: Vec<String>,
    pub cursor: (usize, usize),
}

#[derive(Debug, Deserialize)]
pub struct BufferChangeMessage {
    pub line: usize,
    pub new_text: String,
}
