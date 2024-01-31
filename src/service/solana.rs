pub async fn get_balance() -> Result<String, reqwest::Error> {
    reqwest::get("http://127.0.0.1:3000/getBalance")
        .await
        .unwrap()
        .text()
        .await
}
