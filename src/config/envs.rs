use dotenv::dotenv;
use std::{env, u16};

pub struct Envs {
    pub port: u16,
}

impl Envs {
    pub fn new() -> Self {
        dotenv().ok();

        let port_str = env::var("PORT").expect("PORT must be set in .env");
        let port: u16 = port_str.parse().expect("PORT must be a valid number");

        Envs { port }
    }
}
