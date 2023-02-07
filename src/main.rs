use anyhow::Result;
use dotenvy::{dotenv, var};
use megalodon::{generator, Megalodon, SNS};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let url = var("APP_BASE").expect("APP_BASE env variable should be defined");
    let token = var("APP_TOKEN").expect("APP_TOKEN env variable should be defined");

    let client = gen_client(url, token);
    let response = client.post_status("Hello, world!".into(), None).await?;
    println!("{response:#?}");
    Ok(())
}

fn gen_client(url: String, token: String) -> Box<dyn Megalodon + Send + Sync> {
    generator(SNS::Mastodon, url, Some(token), None)
}
