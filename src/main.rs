#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket::State;

#[macro_use]
extern crate log;

mod bridge;
mod encryption;
mod logging;

struct ConfigState {
    slack_url: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RawSlackUrl<'r> {
    slack_url: &'r str,
}

///
/// Generate a new webhook url
///
/// Encrypts the `slack_url` json input, for use later with [sms_with_url]
#[post("/get_webhook", data = "<raw_slack_url>")]
fn get_webhook(raw_slack_url: Option<Json<RawSlackUrl<'_>>>) -> Result<String, (Status, String)> {
    // Error handling for the form input validation
    if raw_slack_url.is_none() {
        return Err((
            Status::BadRequest,
            "Form data validation failed".to_string(),
        ));
    };
    let encrypted_url = encryption::encrypt_url(raw_slack_url.unwrap().slack_url);

    Ok(encrypted_url)
}

///
/// The webhook to be called by Twillio
///
/// This version (with url) accepts an encrypted slack webhook url, and sends the message
/// from Twillio to that Slack workspace. The encryption is to enhance security, so that
/// your slack webhook url isn't exposed.
#[post("/sms/<slack_url>", data = "<input>")]
async fn sms_with_url(
    input: Option<Form<bridge::TwilioSMSWebhookBody<'_>>>,
    slack_url: String,
) -> Result<String, (Status, String)> {
    info!("/sms/<slack_url> route called");

    let decrypted = encryption::decrypt_url(slack_url);

    // Handle decryption errors with the slack webhook url
    if (decrypted.is_err()) {
        return Err((
            Status::BadRequest,
            "Failed to decrypt slack url. Are you sure you have the right callback url?"
                .to_string(),
        ));
    };

    // Error handling for the form input validation
    if input.is_none() {
        return Err((
            Status::BadRequest,
            "Form data validation failed".to_string(),
        ));
    };

    bridge::bridge_sms_to_slack(decrypted.unwrap(), input.unwrap()).await
}

///
/// The webhook to be called by Twillio
///
/// This version of the webhook accepts no inputs, and relies on the `APP_SLACK_URL`
/// environment variable. This is to be used if you want the service to only work for
/// a specific slack url
#[post("/sms", data = "<input>")]
async fn sms(
    input: Option<Form<bridge::TwilioSMSWebhookBody<'_>>>,
    state: &State<ConfigState>,
) -> Result<String, (Status, String)> {
    info!("/sms route called");

    // Error handling for the form input validation
    if input.is_none() {
        return Err((
            Status::BadRequest,
            "Form data validation failed".to_string(),
        ));
    }

    let maybe_url = state.slack_url.clone();
    match &maybe_url as &str {
        "" => Err((
            Status::InternalServerError,
            "Expected a slack webhook URL to be configured in the environment".to_string(),
        )),
        _ => bridge::bridge_sms_to_slack(maybe_url, input.unwrap()).await,
    }
}

#[launch]
fn rocket() -> _ {
    logging::setup_logger().expect("Failed to start logger");

    let config = config::Config::builder()
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let env_slack_url = match config.get_string("slack_url") {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };

    println!("Starting HTTP server");
    rocket::build()
        .manage(ConfigState {
            slack_url: env_slack_url,
        })
        .mount("/", routes![sms, sms_with_url, get_webhook])
}
