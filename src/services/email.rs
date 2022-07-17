use crate::{
    config::Config,
    error::{OstrichError, OstrichErrorType},
};
use sendgrid_async::{Address, Client, Content, Message, Personalization};
use std::sync::Arc;

pub fn get_email_client(config: Arc<Config>) -> Client {
    let api_key = config.email.api_key.clone();

    let sg = Client::new(api_key);
    sg
}

pub async fn send_email(
    client: &Client,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), OstrichError> {
    let send_to = Address::new().set_email(to).set_name(to);
    let send_from = Address::new().set_email(from).set_name(from);

    let p = Personalization::new().add_to(send_to);

    let m = Message::new()
        .set_from(send_from)
        .set_subject(subject)
        .add_content(Content::new().set_content_type("text/html").set_value(body))
        .add_personalization(p);

    client.send_message(&m).await.map_err(|_| {
        OstrichError::new(
            format!("Could not send email to {}", to),
            OstrichErrorType::EmailError,
        )
    })?;

    Ok(())
}

pub async fn send_zillow_listings_email(
    client: &Client,
    config: Arc<Config>,
    to: &str,
    body: &str,
) -> Result<(), OstrichError> {
    log::info!("Sending Email to {}!", to);
    let from = config.email.from.clone();
    let subject = "Zillow Results";
    log::debug!(
        "from:{}\nto:{}\nsubject:{}\nbody:{}",
        from,
        to,
        subject,
        body
    );
    send_email(client, &from, to, subject, body).await
}
