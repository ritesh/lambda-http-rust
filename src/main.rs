use lambda_runtime::{service_fn, Context, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(hello);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn hello(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");
    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;

    #[tokio::test]
    async fn hello_handles() {
        let event = LambdaEvent {
            payload: json!({"message" : "Hello, world!"}),
            context: Context::default(),
        };
        let expected = json!({
            "message": "Hello, world!"
        });

        let response = hello(event).await.expect("expected Ok(_) value");
        assert_eq!(response, expected);
    }
    #[tokio::test]
    async fn hello_handles_with_name() {
        let event = LambdaEvent {
            payload: json!({"firstName" : "Ritesh"}),
            context: Context::default(),
        };
        let expected = json!({
            "message": "Hello, Ritesh!"
        });

        let response = hello(event).await.expect("expected Ok(_) value");
        assert_eq!(response, expected);
    }
}
