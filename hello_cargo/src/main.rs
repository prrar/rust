use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use ureq::Agent;

const GET_IP_URL: &str = "https://ipv4.icanhazip.com";
const API_URL: &str = "https://api.cloudflare.com/client/v4";

#[derive(Debug, Deserialize)]
struct Zone {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: String,
    name: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Response<T> {
    result: Vec<T>,
}

fn get_zones(agent: &Agent, token: &str) -> Result<Vec<Zone>, ureq::Error> {
    let header = format!("Bearer {token}");
    let url: String = format!("{API_URL}/zones?per_page=50&status=active");
    let response: Response<Zone> = agent.get(url)
        .header("Authorization", header)
        .call()?
        .body_mut()
        .read_json()?;

    Ok(response.result)
}

fn get_records(agent: &Agent, token: &str, zone_id: &str, domain: &str) -> Result<Vec<Record>, ureq::Error> {
    let header = format!("Bearer {token}");
    let url: String = format!("{API_URL}/zones/{zone_id}/dns_records?type=A&name.exact={domain}");
    let response: Response<Record> = agent.get(url)
        .header("Authorization", header)
        .call()?
        .body_mut()
        .read_json()?;

    Ok(response.result)
}

fn update_record(agent: &Agent, token: &str, zone_id: &str, record: &Record, new_ip: &str) -> Result<(), ureq::Error> {
    let header = format!("Bearer {token}");
    let url: String = format!("{API_URL}/zones/{zone_id}/dns_records/{}",record.id);
    let new_record = Record {
        id: record.id.clone(),
        name: record.name.clone(),
        content: new_ip.to_string(),
    };

    agent.patch(url)
        .header("Authorization", header)
        .send_json(&new_record)?;

    Ok(())
}

fn get_ip(agent: &Agent) -> Result<String, ureq::Error> {
    let body = agent.get(GET_IP_URL)
        .call()?
        .body_mut()
        .read_to_string()?;
    Ok(body.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let token = std::env::var("CF_API_TOKEN")
        .map_err(|_| "CF_API_TOKEN not defined")?
        .trim()
        .to_string();

    if token.is_empty() {
        return Err("CF_API_TOKEN is empty".into());
    }

    let domain = std::env::var("CF_DOMAIN")
        .map_err(|_| "CF_DOMAIN not defined")?
        .trim()
        .trim_end_matches('.')
        .to_lowercase();

    if domain.is_empty() {
        return Err("CF_DOMAIN is empty".into());
    }

    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();

    for zone in get_zones(&agent, &token)? {
        let records = get_records(&agent, &token, &zone.id, &domain)?;
        if let Some(record) = records.first() {
            let ip = get_ip(&agent)?;
            if ip == record.content {
                println!("IP address is the same as the record content. No update needed.");
                return Ok(());
            }
            update_record(&agent, &token, &zone.id, record, &ip)?;
            println!("Updated record: {}: {} -> {}", record.name, record.content, ip);
            return Ok(());
        }
    }
    Err(format!("Record {} not found", domain).into())
}