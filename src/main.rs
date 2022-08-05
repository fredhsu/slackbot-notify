use clap::Parser;
use reqwest::{Response, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::str;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // NATS server address
    #[clap(long, default_value_t = String::from("localhost"))]
    nats_host: String,
    // NATS subject to listen on
    #[clap(long, default_value_t = String::from("slackbot.notify"))]
    nats_subject: String,
    // Location of the slack config json file
    #[clap(long, default_value_t = String::from("tokens/slack.json"))]
    slack_config_file: String,
}

#[derive(Serialize)]
struct MessagePayload {
    channel: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    pub token: String,
    pub webhook_url: String,
    pub channel: String,
}

fn read_json_config(filename: &str) -> Config {
    let file = fs::read_to_string(filename).expect("Unable to read config file");
    serde_json::from_str(&file).expect("JSON is bad")
}

async fn post_message(msg: &str) -> Result<Response> {
    let slack_config = read_json_config("tokens/slack.json");
    let message = MessagePayload {
        channel: slack_config.channel,
        text: msg.to_string(),
    };
    let client = reqwest::Client::new();
    Ok(client
        .post(slack_config.webhook_url)
        .bearer_auth(slack_config.token)
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
