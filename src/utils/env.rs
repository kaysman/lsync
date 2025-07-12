use dotenvy::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_client_id() -> String {
    env::var("CLIENT_ID").expect("CLIENT_ID not set in .env")
}

pub fn get_client_secret() -> String {
    env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set in .env")
}
