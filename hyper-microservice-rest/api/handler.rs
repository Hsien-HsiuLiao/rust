use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from("Hello from Vercel Rust!"))?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
} 