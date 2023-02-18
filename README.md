## About

SlackMS solves a simple, but annoying problem: SMS 2FA codes for accounts shared by a team. SlackMS listens to the webhook emitted by Twilio when an SMS message is received and forwards it along to Slack.

The app is designed to be easily self-hostable using the Docker container.

## Getting Started

## Installation

## Usage

### POST `/get_webhook`

The `get_webhook` route allows you to set up your webhook for Twillio, using your existing Slack
webhook.

#### Input

```json
{
  "slack_url": "https://hooks.slack.com/services/.../.../..."
}
```

#### Response

- `200`: An encrypted version of your Slack webhook URL
- `400`: Bad Request, you probably didn't input the Slack URL properly

See Usage above for instructions on creating your Twilio Webhook using this encrypted Slack URL

## Roadmap

## Contributing
