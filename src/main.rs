use std::error::Error;

use dotenvy::{dotenv, var};
use elefren::{Data, Mastodon, MastodonClient};

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mastodon = Mastodon::from(Data {
        base: var("APP_BASE").expect("Failed getting app base").into(),
        client_id: var("APP_ID").expect("Failed getting app Id").into(),
        client_secret: var("APP_SECRET").expect("Failed getting app secret").into(),
        redirect: "/".into(),
        token: var("APP_TOKEN").expect("Failed getting app token").into(),
    });

    let you = mastodon.verify_credentials()?;

    println!("{you:#?}");

    Ok(())
}
