use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};

pub struct CloudFlareApi {
    api_token: String,
    client: reqwest::Client,
}
impl CloudFlareApi {
    pub fn new(token: &String) -> Self {
        Self {
            api_token: token.clone(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn get_dns_records(&self, zone_id: &String, protocol: &Protocol, domain: &String)
                                 -> Result<CloudFlareDNSListResponse, String>
    {
        // let URL: String = "https://api.cloudflare.com/client/v4/zones/".to_string() + &*zone_id + "/dns_records";
        let url: String = format!("https://api.cloudflare.com/client/v4/zones/{0}/dns_records", zone_id);
        let params: HashMap<&str, String> = HashMap::from([
            ("match", "all".to_string().clone()),
            ("type", protocol.to_string().clone()),
            ("name", domain.clone())
        ]);
        match self.client.get(url)
            .header("Authorization", format!("Bearer {0}", self.api_token))
            .query(&params)
            .send().await {
            Ok(response) => {
                let output = response.text().await.unwrap();
                match serde_json::from_str::<CloudFlareDNSListResponse>(&output) {
                    Ok(json) => {
                        Ok(json)
                    }
                    Err(error) => {
                        Err(format!("parsing response json failed. {}", error))
                    }
                }
            }
            Err(error) => {
                Err(format!("get_dns_records failed. {}", error))
            }
        }
    }
    pub async fn update_ip(&self, ip_addr: &String, dns_record_id: &String, target: &UpdateTarget)
                           -> Result<(), String> {
        let url: String = format!("https://api.cloudflare.com/client/v4/zones/{0}/dns_records/{1}", target.zone_id, dns_record_id);
        let request_body: CloudFlareUpdateDNSRequest = CloudFlareUpdateDNSRequest {
            content: ip_addr.to_string(),
            name: target.domain.clone(),
            ttype: target.record_type.to_string(),
            id: dns_record_id.to_string(),
        };
        match self.client.patch(url)
            .header("Authorization", format!("Bearer {0}", self.api_token))
            .body::<String>(serde_json::to_string(&request_body).unwrap())
            .send().await {
            Ok(_response) => {
                Ok(())
            }
            Err(error) => {
                Err(format!("Updating ip failed. {}", error))
            }
        }
    }
}


#[derive(Deserialize)]
pub struct CloudFlareDNSUpdateRequest {
    content: String,
    name: String,
    #[serde(rename = "type")]
    ttype: String,
}
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct UpdateTarget {
    pub domain: String,
    pub record_type: Protocol,
    pub zone_id: String,
}
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Protocol {
    A,
    AAAA,
}
impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::A => write!(f, "A"),
            Protocol::AAAA => write!(f, "AAAA"),
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct CloudFlareDNSListResponse {
    // pub errors: Vec<String>,
    // pub messages: Vec<String>,
    pub success: bool,
    // pub result_info: CloudFlareDNSListResponsePages,
    pub result: Vec<CloudFlareDNSListResponseResult>,
}
#[derive(Debug, Deserialize)]
pub struct CloudFlareDNSListResponsePages {
    pub count: i32,
    pub page: i32,
    pub per_page: i32,
    pub total_count: i32,
}
#[derive(Debug, Deserialize)]
pub struct CloudFlareDNSListResponseResult {
    pub content: String,
    pub name: String,
    pub proxied: bool,
    pub comment: Option<String>,
    #[serde(rename = "type")]
    pub ttype: String,
    pub created_on: String,
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CloudFlareUpdateDNSRequest {
    //The ip address
    content: String,
    //dns record name such as example.com
    name: String,
    #[serde(rename = "type")]
    ttype: String,
    id: String,
}