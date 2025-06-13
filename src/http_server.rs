use axum::{
    Json, Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use comrak::{Options, markdown_to_html};

use crate::SharedState;
const GITHUB_MARKDOWN_CSS: &str = include_str!("../assets/github-markdown-dark.css");

pub async fn run_http_server(state: SharedState) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(serve_preview))
        .route("/messages", get(get_state))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🌐 Serving preview at http://localhost:3000");

    if webbrowser::open("http://localhost:3000").is_ok() {
        println!("🚀 Browser launched");
    }

    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_preview(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();

    let mut options = comrak::Options::default();
    options.extension.alerts = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some("---".into());
    options.extension.alerts = true;

    let html = markdown_to_html(&state.content.join("\n"), &options);

    let full = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Live Preview</title>
  <style>{GITHUB_MARKDOWN_CSS}</style>
  <style>
    body {{
      margin: 2rem auto;
      max-width: 800px;
      padding: 0 1rem;
    }}
  </style>
</head>
<body class="markdown-body">
  {html}
</body>
</html>"#
    );
    Html(full)
}

async fn get_state(
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> impl IntoResponse {
    let messages = {
        let state = state.read().unwrap();
        state.messages.clone()
    };
    Json(messages)
}
