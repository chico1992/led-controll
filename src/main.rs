#[macro_use]
extern crate dotenv_codegen;

use std::collections::HashMap;

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hue(HueArgs),
}

#[derive(Args)]
struct HueArgs {
    #[command(subcommand)]
    command: HueCommands,
}

#[derive(Subcommand)]
enum HueCommands {
    List,
    On {
        group: String,
        color: Option<String>,
    },
    Off {
        group: String,
    },
}

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

fn get_hue_url() -> String {
    format!(
        "https://{}/api/{}/groups",
        dotenv!("HUE_IP"),
        dotenv!("HUE_USER")
    )
}

async fn get_hue() -> Result<HashMap<String, Group>, Box<dyn std::error::Error>> {
    let resp: HashMap<String, Group> = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(get_hue_url())
        .send()
        .await?
        .json()
        .await?;
    return Ok(resp);
}

async fn toggle_hue_group(group: String, state: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("on", state);
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .put(format!("{}/{}/action", get_hue_url(), group))
        .json(&map)
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hue(hue) => {
            let hue_cmd = hue.command;
            match hue_cmd {
                HueCommands::List {} => {
                    let resp = get_hue().await?;
                    println!("{:#?}", resp);
                }
                HueCommands::On { group, color } => {
                    toggle_hue_group(group, true).await?;
                }
                HueCommands::Off { group } => {
                    toggle_hue_group(group, false).await?;
                }
                _ => {
                    println!("need to be implemented")
                }
            }
        }
    }
    Ok(())
}
