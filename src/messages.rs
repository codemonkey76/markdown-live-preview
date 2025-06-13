use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum Message {
    Init(InitMessage),
    BufferChange(BufferChangeMessage),
    CursorMoved(CursorMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CursorMessage {
    pub cursor: (usize, usize),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitMessage {
    pub content: Vec<String>,
    pub cursor: (usize, usize),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BufferChangeMessage {
    pub line: usize,
    pub new_text: String,
}
