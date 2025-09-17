use anyhow::{Result, bail};
use reqwest::{Client, Url};
use serde::Serialize;

#[derive(Serialize)]
pub struct DiscordWebhookPayload<'a> {
    pub username: String,
    pub content: String,
    pub embeds: Vec<DiscordWebhookEmbedPayload<'a>>,
}

#[derive(Serialize)]
pub struct DiscordWebhookEmbedPayload<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub color: usize,
    pub timestamp: String,
}

impl DiscordWebhookPayload<'_> {
    pub async fn post_to_webhook_url(&self, client: &Client, url: &Url) -> Result<()> {
        let response = client
            .post(url.as_str())
            .header("Content-Type", "application/json")
            .json(&self)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("failed with response: {:?}", response.text().await?);
        }

        Ok(())
    }
}
