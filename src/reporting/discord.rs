use std::collections::HashMap;

const MAX_REPORT_LENGTH: usize = 1900;

pub struct DiscordReporter {
    client: reqwest::Client,
    webhook: reqwest::Url,
}

impl DiscordReporter {
    pub fn new() -> Result<Self, String> {
        let client = reqwest::Client::new();

        let webhook = std::env::var("DISCORD_WEBHOOK_URL")
            .map_err(|_| "'DISCORD_WEBHOOK_URL' is not set.")?;
        let webhook =
            reqwest::Url::parse(&webhook).map_err(|e| format!("Failed to parse webhook: {}", e))?;

        Ok(Self { client, webhook })
    }

    pub async fn send_report(&self, message: &str) -> Result<(), String> {
        let max_message_length = MAX_REPORT_LENGTH.min(message.len());

        let mut map = HashMap::new();
        map.insert("content", message[..max_message_length].to_string());

        match self
            .client
            .post(self.webhook.clone())
            .json(&map)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to send discord report: {}", e)),
        }
    }
}
