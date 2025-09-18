use crate::{
    AppState,
    discord::{DiscordWebhookEmbedPayload, DiscordWebhookPayload},
};
use axum::{Json, extract::State, http::StatusCode};
use axum_client_ip::ClientIp;
use axum_extra::{TypedHeader, headers::UserAgent};
use serde::Deserialize;
use sqlx::query;
use tracing::{error, warn};

#[derive(Deserialize)]
pub struct QuestionPayload {
    pub content: String,
}

pub async fn add_ask(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(payload): Json<QuestionPayload>,
) -> StatusCode {
    let content = strip_markdown(payload.content.trim());
    let ip = ip.to_string();
    let user_agent = user_agent.as_str();

    // Validate content length.
    if content.len() < state.page_question_min_length
        && content.len() > state.page_question_max_length
    {
        warn!("A question was submitted but it failed to validate length requirements, rejecting.");
        return StatusCode::BAD_REQUEST;
    }

    // Add question into asks database.
    // Note: expiry is handled for us by a background task.
    if let Err(e) = query!(
        "INSERT INTO asks (content, ipAddress, userAgent) VALUES (?, ?, ?)",
        content,
        ip,
        user_agent
    )
    .execute(state.database.pool())
    .await
    {
        error!("Failed to insert question into the database: {e:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // Send question to Discord.
    if let Err(e) = (DiscordWebhookPayload {
        username: format!("{}'s Fragekasten", state.page_owner_name),
        content: format!("<@{}>", state.discord_user_id),
        embeds: vec![DiscordWebhookEmbedPayload {
            title: "New Question",
            description: &content,
            color: 3447003, // (hex: #3498db)
            timestamp: chrono::Utc::now().to_rfc3339(),
        }],
    })
    .post_to_webhook_url(&state.reqwest_client, &state.discord_webhook_url)
    .await
    {
        error!("Failed to send question via Discord webhook: {e:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::CREATED
}

/// Remove most markdown from the input and return an almost-plain sanitised string.
fn strip_markdown(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut output = String::new();
    for event in parser {
        if let pulldown_cmark::Event::Text(text) | pulldown_cmark::Event::Code(text) = event {
            output.push_str(&text);
        }
    }
    output
}
