use std::env;

use reqwest::{RequestBuilder, StatusCode};
use rocket::{Build, Rocket};
use serde::Deserialize;
use serde_json::json;

pub trait RocketAddLoginService {
    fn add_login_service(self) -> Self;
}

impl RocketAddLoginService for Rocket<Build> {
    fn add_login_service(self) -> Self {
        self.manage(LoginService::new())
    }
}

#[derive(Debug, Clone)]
pub struct LoginService {
    client: reqwest::Client,

    api_path: String,
}

#[derive(Debug, Deserialize)]
pub struct UserId {
    id: i32,
}

impl LoginService {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        let api_address = env::var("LOGIN_SERVICE_ADDRESS").expect("DATABASE_URL must be set");

        let api_path = api_address + "api/v1/";
        Self { client, api_path }
    }

    fn request_builder_post(&self, path: &str) -> RequestBuilder {
        self.client.post(format!("{}/{}", self.api_path, path))
    }

    pub async fn login(&self, email: &str, password: &str) -> Option<i32> {
        let result = self
            .request_builder_post("user/login")
            .json(&json!({
                "email": email.to_string(),
                "password": password.to_string(),
            }))
            .send()
            .await;

        match result {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    let j: UserId = response
                        .json()
                        .await
                        .expect("Login service json validation failure");
                    Some(j.id)
                }
                _ => {
                    println!(
                        "Failed to access login service: \nStatus: {}",
                        response.status()
                    );
                    None
                }
            },
            Err(err) => {
                println!("Error to access login service: {:?}", err);
                None
            }
        }
    }
}
