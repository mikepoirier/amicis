use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use leptos::{component, ssr::render_to_string, view, IntoView};

use super::{health_status, HealthStatus};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_root))
        .route("/status", get(get_status))
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <!doctype html>
        <html lang="en">
        <head>
            <title>Amicis</title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="h-full container mx-auto">
            <h1 class="text-2xl font-semibold">Amicis</h1>
            <div id="status" hx-get="/status" hx-trigger="load" hx-swap="outerHTML" />
        </body>
        </html>
    }
}

#[component]
fn StatusView(status: HealthStatus) -> impl IntoView {
    view! {
        <div id="status" hx-get="/status" hx-trigger="every 30s" hx-swap="outerHTML">"Status: "{status.to_string()}</div>
    }
}

async fn get_root() -> impl IntoResponse {
    Html(render_to_string(|| view! { <HomePage /> }).to_string())
}

async fn get_status() -> impl IntoResponse {
    let status = health_status().await;
    Html(render_to_string(|| view! { <StatusView status /> }).to_string())
}
