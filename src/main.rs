use clap::Parser;
use reqwest::{Response, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{io::Read, str};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // NATS server address
    #[clap(long, default_value_t = String::from("localhost"))]
    nats_host: String,
    // NATS subject to listen on
    #[clap(long, default_value_t = String::from("slackbot.notify"))]
    nats_subject: String,
}

#[derive(Serialize, Deserialize)]
struct MessagePayload {
    channel: String,
    text: String,
}

fn read_config(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

async fn post_message(msg: &str) -> Result<Response> {
    let slack_token = read_config("tokens/slack.token");
    let webhook_url =
        "https://hooks.slack.com/services/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";
    let channel = "C02E3V00VT6".to_string();
    let message = MessagePayload {
        channel,
        text: msg.to_string(),
    };
    let client = reqwest::Client::new();
    Ok(client
        .post(webhook_url)
        .bearer_auth(slack_token)
        .json(&message)
        .send()
        .await?)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = nats::connect(cli.nats_host).unwrap();
    let subscriber = client.subscribe(&cli.nats_subject).unwrap();
    while let Some(message) = subscriber.next() {
        println!("received message {:?}", &message);
        let resp = post_message(str::from_utf8(&message.data).unwrap());
        // TODO check result of posting message
        println!("{:?}", resp.await.unwrap().text().await.unwrap());
    }
}
