use std::env::var;

use client::Client;
mod client;
pub struct AppicationConfig<'a> {
    pub oauth_token: &'a str,
}

fn main() {
    let token = var("OAUTH").expect("Unable to find OAUTH in env");
    let app_conf = AppicationConfig {
        oauth_token: &token
    };

    let _client = Client { app_conf: app_conf };
}
