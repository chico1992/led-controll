#[macro_use]
extern crate dotenv_codegen;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    name: String,
    lights: Vec<String>,
    sensors: Vec<String>,
    #[serde(rename = "type")]
    kind: String,
    state: GroupState,
    recycle: bool,
    class: String,
    action: GroupAction,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroupState {
    all_on: bool,
    any_on: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroupAction {
    on: bool,
    bri: u8,
    hue: u16,
    sat: u8,
    effect: String,
    xy: Vec<f64>,
    ct: usize,
    alert: String,
    colormode: String,
}

#[derive(Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

async fn get_hue() -> Result<HashMap<String, Group>, Box<dyn std::error::Error>> {
    let resp: HashMap<String, Group> = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(format!(
            "https://{}/api/{}/groups",
            dotenv!("HUE_IP"),
            dotenv!("HUE_USER")
        ))
        .send()
        .await?
        .json()
        .await?;
    return Ok(resp);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = get_hue().await?;
    println!("{:#?}", resp);
    Ok(())
}
