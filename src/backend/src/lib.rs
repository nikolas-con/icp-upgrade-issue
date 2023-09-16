use candid::Principal;

use ic_cdk::api::management_canister::main::*;


#[ic_cdk_macros::update]
fn upgrade(wasm: Vec<u8>) {
    let canister_id = ic_cdk::id();

    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        wasm_module: wasm,
        canister_id: canister_id,
        arg: ic_cdk::api::id().as_slice().to_vec(),
    };

    ic_cdk_timers::set_timer(std::time::Duration::from_secs(5), ||
    {
        ic_cdk::spawn(async move{ ic_cdk::notify::<_>(Principal::management_canister(), "install_code", (install_arg,)).unwrap() })
    });
}

#[ic_cdk_macros::query]
fn version() -> String {
    "v1".to_owned()
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade");
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade(){
    ic_cdk::println!("post_upgrade");
}
