use log::error;

pub async fn get_ip() -> Result<String, &'static str> {
    match reqwest::get("https://api.ipify.org/").await {
        Ok(response) => {
            match response.text().await {
                Ok(ip) => {
                    Ok(ip)
                }
                Err(error) => {
                    error!("{0}]", &error);
                    Err("No ip addr")
                }
            }
        }
        Err(error) => {
            error!("{0}]", &error);
            Err("No ip addr")
        }
    }
}