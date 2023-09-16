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

    // get agent url
    let networks = HashMap::from([
        ("ic".to_owned(), "https://ic0.app".to_owned()),
        ("local".to_owned(), "http://127.0.0.1:8000".to_owned())
    ]);
    let network_name_default = "local".to_owned();
    let network_name_opt = args.iter().position(|r| r == "--network").map(|i| args.get(i + 1).unwrap());
    let network_name = network_name_opt.unwrap_or(&network_name_default);
    let agent_url = networks.get(network_name).unwrap();
    dbg!(agent_url);

    return;

    // get canister id
    let file = File::open(".dfx/local/canister_ids.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let networks = json.get("backend").unwrap();
    let local = networks.get("local").unwrap().as_str().unwrap();
    let canister_id = local.to_string();

    // get agent
    let canister = Principal::from_text(&canister_id).unwrap();
    let agent = Agent::builder().with_url(agent_url).build().unwrap();
    agent.fetch_root_key().await.unwrap();
    
    // upgrade canister
    agent.update(&canister, "upgrade").with_arg(Encode!(&WASM.to_vec()).unwrap()).call_and_wait().await.unwrap();
    println!("Upgrade started");

}
