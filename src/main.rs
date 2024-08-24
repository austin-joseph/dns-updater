mod services;

use std::env;
use services::{
    ip_resolver_service,
    cloudflare_service::{
        CloudFlareApi
    },
};
use log::{debug, error, info, trace};
use serde::Deserialize;
use crate::services::cloudflare_service::{UpdateTarget};

#[tokio::main]
async fn main() {
    let app_config: &AppConfig = &load_config_from_file();
    env_logger::init();
    info!("Application Start");
    update_dns_records(&app_config.api_token, &app_config.update_targets).await;
    info!("Stopping");
}

async fn update_dns_records(api_token: &String, update_targets: &Vec<UpdateTarget>) {
    let ipaddr = match ip_resolver_service::get_ip().await {
        Ok(address) => {
            address
        }
        Err(_) => {
            error!("Unable to retrieve ip address");
            panic!("Unable to retrieve ip address")
        }
    };
    info!("Local address {}", ipaddr);

    let cloudflare_api = CloudFlareApi::new(api_token);
    let target_list: &Vec<UpdateTarget> = update_targets;
    for target in target_list {
        let dns_records = cloudflare_api.get_dns_records(&target.zone_id, &target.record_type, &target.domain).await.unwrap();
        info!("Found {} records", dns_records.result.len());
        for dns_record in &dns_records.result {
            if (target.record_type.to_string().ne(&dns_record.ttype)
                || target.domain.to_string().ne(&dns_record.name)
            ) {
                trace!("Skipping dns record from cloudflare that doesnt match current target. {}", target.domain);
                continue;
            }
            if (dns_record.content.eq(&ipaddr)
            ) {
                debug!("Ip address for {} matches whats in cloudflare. Local Ip: {} == Cloudflare Registered Ip: {}", target.domain, ipaddr,dns_record.content );
                continue;
            }
            info!("Ip address mismatch for domain {} attempting to update. Local IP {} Registered Ip {}", target.domain, ipaddr, dns_record.content );

            match cloudflare_api.update_ip(&ipaddr, &dns_record.id, &target).await {
                Ok(_) => {
                    info!("Ip address updated for {} to {}", target.domain, ipaddr)
                }
                Err(e_message) => {
                    error!("{}", e_message);
                    continue;
                }
            }
        }
    }
}
fn load_config_from_file() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    let config_loc = match args.get(1) {
        Some(loc) => {
            loc
        }
        None => {
            panic!("Config file not specified in first argument");
        }
    };

    let imported_config = config::Config::builder()
        .add_source(config::File::with_name(config_loc))
        .add_source(config::Environment::with_prefix("NLH"))
        .build()
        .unwrap();

    imported_config
        .try_deserialize::<AppConfig>()
        .unwrap()
}

#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfig {
    api_token: String,
    update_targets: Vec<UpdateTarget>,
}