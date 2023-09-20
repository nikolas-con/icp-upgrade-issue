use candid::Encode;
use candid::Principal;
use ic_agent::{Agent, identity::*, agent::*};
use std::io::Cursor;
use ic_cdk::api::management_canister::main::{InstallCodeArgument, CanisterInstallMode};

use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::path::*;
use std::fs;

const WASM: &[u8] = include_bytes!("../../build/backend_v2.wasm");

// cargo run -p upload -- --command user-upgrading
// cargo run -p upload -- --network ic --command self-upgrading
#[tokio::main]
async fn main() {
    // get network
    let args: Vec<String> = env::args().collect();
    let network_name_default = "local".to_owned();
    let network_name_opt = args.iter().position(|r| r == "--network").map(|i| args.get(i + 1).unwrap());
    let command_opt = args.iter().position(|r| r == "--command").map(|i| args.get(i + 1).unwrap());

    // get config
    let networks = HashMap::from([
        ("ic".to_owned(), ("https://ic0.app".to_owned(), "canister_ids.json".to_owned())),
        ("local".to_owned(), ("http://127.0.0.1:8000".to_owned(), ".dfx/local/canister_ids.json".to_owned()))
    ]);
    let network_name = network_name_opt.unwrap_or(&network_name_default);
    let (agent_url, canister_path) = networks.get(network_name).unwrap();

    if command_opt == Some(&"self-upgrading".to_owned()) {
        self_upgrading(agent_url, canister_path, network_name).await;
    } else if  command_opt == Some(&"user-upgrading".to_owned()) {
        user_upgrading(agent_url, canister_path, network_name).await;
    } else {
        println!("No command")
    }
}


async fn self_upgrading(agent_url: &String, canister_path: &String, network_name: &String) {
    // get agent
    let transport = http_transport::ReqwestHttpReplicaV2Transport::create(agent_url).unwrap();
    let agent = Agent::builder().with_transport(transport).build().unwrap();
    agent.fetch_root_key().await.unwrap();

    // get canister
    let file = File::open(canister_path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let networks = json.get("backend").unwrap();
    let canister_id = networks.get(network_name).unwrap().as_str().unwrap();
    let canister = ic_agent::export::Principal::from_text(&canister_id.to_string()).unwrap();

    // upgrade canister
    agent.update(&canister, "upgrade").with_arg(Encode!(&WASM.to_vec()).unwrap()).call_and_wait().await.unwrap();
    println!("Upgrade started");
}

async fn user_upgrading(agent_url: &String, canister_path: &String, network_name: &String) {

    // get identity
    let home_path = std::env::var("HOME").unwrap();
    let pem_file = Path::new(&home_path).join(".config").join("dfx").join("identity").join("default").join("identity.pem");
    let pem_file = fs::read(&pem_file).unwrap();
    let identity = BasicIdentity::from_pem(Cursor::new(pem_file)).unwrap();
    
    // get agent
    let transport = http_transport::ReqwestHttpReplicaV2Transport::create(agent_url).unwrap();
    let agent = Agent::builder().with_transport(transport).with_identity(identity).build().unwrap();
    agent.fetch_root_key().await.unwrap();

    // get canister
    let file = File::open(canister_path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let networks = json.get("backend").unwrap();
    let canister_id = networks.get(network_name).unwrap().as_str().unwrap();
    let canister = Principal::from_text(canister_id).unwrap();

    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        wasm_module: WASM.to_vec(),
        canister_id: canister,
        arg: canister.as_slice().to_vec(),
    };
    

    // upgrade canister
    agent.update(&Principal::management_canister(), "install_code").with_arg(Encode!(&install_arg).unwrap()).call_and_wait().await.unwrap();
    println!("Upgrade started");
}