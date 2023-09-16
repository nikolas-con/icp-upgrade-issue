use candid::Encode;

use ic_agent::{Agent, export::Principal};

use std::fs::File;

const WASM: &[u8] = include_bytes!("../../build/backend_v2.wasm");

// cargo run -p upload
#[tokio::main]
async fn main() {

    // get canister id
    let file = File::open(".dfx/local/canister_ids.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let networks = json.get("backend").unwrap();
    let local = networks.get("local").unwrap().as_str().unwrap();
    let canister_id = local.to_string();

    // get agent
    let canister = Principal::from_text(&canister_id).unwrap();
    let agent_url = "http://127.0.0.1:8000".to_string();
    let agent = Agent::builder().with_url(agent_url).build().unwrap();
    agent.fetch_root_key().await.unwrap();
    
    // upgrade canister
    agent.update(&canister, "upgrade").with_arg(Encode!(&WASM.to_vec()).unwrap()).call_and_wait().await.unwrap();
    println!("Upgrade started");

}
