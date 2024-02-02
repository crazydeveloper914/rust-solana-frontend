use std::collections::HashMap;

const BASE_URL: &str = "http://127.0.0.1:3000/";

pub async fn get_balance() -> Result<String, reqwest::Error> {
    reqwest::get(format!("{}getBalance", BASE_URL))
        .await
        .unwrap()
        .text()
        .await
}

pub async fn transfer_sol(payload: HashMap<String, String>) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    client
        .post(format!("{}transferSols", BASE_URL))
        .json(&payload)
        .send()
        .await
        .unwrap()
        .text()
        .await
}

pub async fn get_5_sols() -> Result<String, reqwest::Error> {
    reqwest::get(format!("{}getSols", BASE_URL))
        .await
        .unwrap()
        .text()
        .await
}