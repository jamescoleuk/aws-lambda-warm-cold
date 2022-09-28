use lambda_http::{service_fn, Error, IntoResponse, Request};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Put things you don't want to do frequently here. E.g.:
    // - creating a database connection pool
    // - accessing secrets
    // - creating HTTP clients
    // - retrieving config from environment variables

    println!();
    println!("Initialization: this only runs on a cold start.");
    println!();

    let id = Uuid::new_v4();
    lambda_http::run(service_fn(|event| execute(event, id))).await?;
    Ok(())
}

pub async fn execute(_: Request, id: Uuid) -> Result<impl IntoResponse, std::convert::Infallible> {
    // Put your core logic in here.

    println!();
    println!("Handler code: this runs on every invocation.");
    println!(
        "Handler code: the ID from the initialization code is: {}",
        id
    );
    println!();

    Ok("Hello warm/cold world!".to_string())
}
