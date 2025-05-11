use dotenvy::dotenv;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    println!("Hello, world!");
    Ok(())
}
