use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use lazy_static::lazy_static;
use serde::{ Deserialize, Serialize };
use serde_json::json;

lazy_static! {
    static ref URL: String = std::env::var("URI").unwrap();
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<EventBridgeEvent<EventDetail>>) -> Result<(), Error> {
    // Extract some useful information from the request

    let detail = event.payload.detail;

    send_discord(vec![detail.status, detail.content].join("\n")).await;

    Ok(())
}


/// This function calls Discord's Webhook to notify error event
async fn send_discord(content: String) {

    let body = json!({
        "content": content
    });

    let client = reqwest::Client::new();
    client
        .post(&String::from(&*URL))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await.expect("TODO: panic message");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[derive(Debug, Serialize, Deserialize)]
struct EventDetail {
    status: String,
    content: String
}