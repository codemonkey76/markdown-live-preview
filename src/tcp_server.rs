use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};

use crate::{
    SharedState,
    messages::{BufferChangeMessage, CursorMessage, InitMessage, Message},
};

pub async fn run_tcp_listener(state: SharedState) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    println!("🔌 Listening for buffer updates on 127.0.0.1:3001");

    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(stream);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                match serde_json::from_str::<Message>(&line) {
                    Ok(Message::Init(InitMessage { content, cursor })) => {
                        let mut s = state.write().unwrap();
                        s.content = content;
                        s.cursor = cursor;
                        println!("✅ Init: {} lines", s.content.len());
                    }
                    Ok(Message::BufferChange(BufferChangeMessage { line, new_text })) => {
                        let mut s = state.write().unwrap();
                        if line < s.content.len() {
                            s.content[line] = new_text;
                        } else {
                            s.content.push(new_text);
                        }
                        println!("✏️ Updated line {line}");
                    }
                    Ok(Message::CursorMoved(CursorMessage { cursor })) => {
                        let mut s = state.write().unwrap();
                        s.cursor = cursor;
                        println!("👉 Cursor moved to {cursor:?}");
                    }
                    Err(e) => eprintln!("❌ Error parsing message: {e}"),
                }
            }

            println!("🛑 Connection closed");
        });
    }
}
