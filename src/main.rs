use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct NameInputEvent {
  pub first_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct HelloWorldResponse {
  pub message: String,
}

async fn func(event: LambdaEvent<NameInputEvent>) -> Result<HelloWorldResponse, Error> {
    let (event, _context) = event.into_parts();

    let hello_world_response = HelloWorldResponse {
        message: format!("Hello, {}! Welcome to rust-lambda.", event.first_name),
    };

    Ok(hello_world_response)
}