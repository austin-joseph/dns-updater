pub async fn get_ip() -> Result<String, String> {
    match reqwest::get("https://api.ipify.org/").await {
        Ok(response) => {
            match response.text().await {
                Ok(ip) => {
                    Ok(ip)
                }
                Err(error) => {
                    Err(format!("[{0}]", &error))
                }
            }
        }
        Err(error) => {
            Err(format!("[{0}]", &error))
        }
    }
}