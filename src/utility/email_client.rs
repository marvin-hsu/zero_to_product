use crate::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;
use tracing::info;
use url::Url;

#[derive(Debug, Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: Url,
    sender: SubscriberEmail,
    api_key: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        api_key: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        Self {
            http_client,
            base_url: base_url.parse().unwrap(),
            sender,
            api_key,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self.base_url.join("/v3/email/send").unwrap();

        let request_body = SendEmailRequest {
            api_key: self.api_key.expose_secret().clone(),
            to: vec![recipient.as_ref().to_string()],
            sender: self.sender.as_ref().to_string(),
            subject: subject.to_string(),
            text_body: content.to_string(),
        };

        self.http_client
            .post(&url.to_string())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct SendEmailRequest {
    pub api_key: String,
    pub to: Vec<String>,
    pub sender: String,
    pub subject: String,
    pub text_body: String,
}

#[cfg(test)]
mod tests {
    use crate::{EmailClient, SubscriberEmail};
    use claims::assert_ok;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Fake;
    use secrecy::Secret;
    use wiremock::matchers::{any, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let token = api_key();
        let email_client = email_client(mock_server.uri(), token.clone());

        Mock::given(method("POST"))
            .and(header("Content-Type", "application/json"))
            .and(path("/v3/email/send"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        // Assert
        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        // Arrange
        let mock_server = MockServer::start().await;
        let token = api_key();
        let email_client = email_client(mock_server.uri(), token.clone());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        // Assert
        claims::assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        // Arrange
        let mock_server = MockServer::start().await;
        let token = api_key();
        let email_client = email_client(mock_server.uri(), token.clone());

        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));
        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        // Assert
        claims::assert_err!(outcome);
    }

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn api_key() -> String {
        Sentence(1..20).fake()
    }

    fn timeout() -> std::time::Duration {
        std::time::Duration::from_millis(200)
    }

    fn email_client(base_url: String, token: String) -> EmailClient {
        EmailClient::new(base_url, email(), Secret::new(token), timeout())
    }
    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);

            if let Ok(body) = result {
                body.get("api_key").is_some()
                    && body.get("sender").is_some()
                    && body.get("subject").is_some()
                    && body.get("text_body").is_some()
                    && body.get("to").unwrap().as_array().unwrap().len() == 1
            } else {
                false
            }
        }
    }
}
