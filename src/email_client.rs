use crate::domain::SubscriberEmail;

pub struct EmailClient {
    client: reqwest::Client,
    sender: SubscriberEmail,
    base_url: String,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            sender,
            base_url,
        }
    }

    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_body: &str,
        _text_body: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
