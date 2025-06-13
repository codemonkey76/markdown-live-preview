use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use comrak::{Options, markdown_to_html};

use crate::SharedState;

pub async fn run_http_server(state: SharedState) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(serve_preview))
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
    let html = markdown_to_html(&state.content.join("\n"), &Options::default());
    Html(html)
}
