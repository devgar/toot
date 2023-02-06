use std::error::Error;

use dotenvy::{dotenv, var};
use megalodon::{generator, SNS, Megalodon};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // env_logger::init();

    let url = var("APP_BASE")
        .expect("APP_BASE env variable should be defined");
    let token = var("APP_TOKEN")
        .expect("APP_TOKEN env variable should be defined");
    // let response = verify_credentials(url.as_str(), token).await?;

    let client = gen_client(url, token);
    let response = client.post_status("Hello, world!".into(), None).await?;
    println!("{response:#?}");
    Ok(())
}

fn gen_client(url: String, token: String) -> Box<dyn Megalodon + Send + Sync> {
    generator(SNS::Mastodon, url, Some(token), None)
}
