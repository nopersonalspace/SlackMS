use rocket::form::Form;
use rocket::http::Status;
use slack_hook2::{PayloadBuilder, Slack};

/// The (relevant) information received by Twilio
#[derive(FromForm)]
pub struct TwilioSMSWebhookBody<'r> {
    /* The message body */
    #[field(name = "Body")]
    body: &'r str,
    /* The sms sender */
    #[field(name = "From")]
    from: &'r str,
    /* The sms receiver */
    #[field(name = "To")]
    to: &'r str,
}

/// This function takes in some Twilio SMS information and forwards it
/// along to Slack via a Slack webhook
pub async fn bridge_sms_to_slack(
    slack_url: String,
    sms_body: Form<TwilioSMSWebhookBody<'_>>,
) -> Result<String, (Status, String)> {
    let slack = Slack::new(slack_url).unwrap();

    let p = PayloadBuilder::new()
        .text(format!("From {}: {}", sms_body.from, sms_body.body))
        .build()
        .unwrap();

    let res = slack.send(&p).await;

    match res {
        Ok(()) => Ok("ok".to_string()),
        Err(x) => {
            error!("{}", x);
            Err((Status::InternalServerError, x.to_string()))
        }
    }
}
