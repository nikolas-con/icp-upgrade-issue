use candid::Encode;

use ic_agent::{Agent, export::Principal};

use std::collections::HashMap;
use std::fs::File;
use std::env;

const WASM: &[u8] = include_bytes!("../../build/backend_v2.wasm");

// cargo run -p upload
// cargo run -p upload -- --network ic
#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    // get network name
    let networks = HashMap::from([
        ("ic".to_owned(), ("https://ic0.app".to_owned(), "canister_ids.json".to_owned())),
        ("local".to_owned(), ("http://127.0.0.1:8000".to_owned(), ".dfx/local/canister_ids.json".to_owned()))
    ]);
    let network_name_default = "local".to_owned();
    let network_name_opt = args.iter().position(|r| r == "--network").map(|i| args.get(i + 1).unwrap());
    let network_name = network_name_opt.unwrap_or(&network_name_default);

    // get agent
    let (agent_url, _) = networks.get(network_name).unwrap();
    let agent = Agent::builder().with_url(agent_url.as_str()).build().unwrap();
    agent.fetch_root_key().await.unwrap();

    // get canister
    let (_, canister_path) = networks.get(network_name).unwrap();
    let file = File::open(canister_path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let networks = json.get("backend").unwrap();
    let canister_id = networks.get(network_name).unwrap().as_str().unwrap();
    let canister = Principal::from_text(&canister_id.to_string()).unwrap();
    
    // upgrade canister
    agent.update(&canister, "upgrade").with_arg(Encode!(&WASM.to_vec()).unwrap()).call_and_wait().await.unwrap();
    println!("Upgrade started");

}
