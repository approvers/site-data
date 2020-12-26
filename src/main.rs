mod list;
mod store;

fn main() {
    use std::env::var;
    let store_url = var("STORE_URL").expect("STOE_URL must set");
    let target_file = var("TARGET_FILE").expect("TARGET_FILE must set");

    let github_token = var("GITHUB_TOKEN").expect("GITHUB_TOKEN must set");
    let discord_token = var("DISCORD_KEY").expect("DISCORD_KEY must set");
    let twitter_token = var("TWITTER_TOKEN").expect("TWITTER_TOKEN must set");

    let store: store::Database = store::load(&store_url);
}
