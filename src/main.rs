use dotenvy::{dotenv, var};
use megalodon::{entities, error, generator, SNS};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // env_logger::init();

    let Ok(url) = var("APP_BASE") else {
        println!("Specify MASTODON_URL!!");
        return
    };
    let Ok(token) = var("APP_TOKEN") else {
        println!("Specify MASTODON_ACCESS_TOKEN!!");
        return
    };
    match verify_credentials(url.as_str(), token).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
async fn verify_credentials(
    url: &str,
    access_token: String,
) -> Result<entities::Account, error::Error> {
    let client = generator(SNS::Mastodon, url.to_string(), Some(access_token), None);
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}
