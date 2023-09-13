use candid::Principal;
use ic_cdk_macros::update;
use ic_cdk::api::management_canister::main::*;

#[update]
fn upgrade() {
    let canister_id = ic_cdk::id();
    let wasm = include_bytes!("../../../build/canister_upgreade_backend.wasm");

    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        wasm_module: wasm.to_vec(),
        canister_id: canister_id,
        arg: vec![]
    };
    ic_cdk::notify::<_>(Principal::management_canister(), "install_code", (install_arg,)).unwrap();
    // ic_cdk::call::<_, ((),)>(Principal::management_canister(), "install_code", (install_arg,)).await.unwrap();
}


#[update]
async fn set_controller() {
    let canister_id = ic_cdk::id();
    let caller = ic_cdk::caller();
    let update_settings_argument = UpdateSettingsArgument{
        canister_id: canister_id, 
        settings: CanisterSettings { 
            controllers: Some(vec![canister_id, caller]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None 
    }};
    ic_cdk::call::<_, ((),)>(Principal::management_canister(), "update_settings", (update_settings_argument,)).await.unwrap();
}


#[update]
fn version_1() {
    ic_cdk::println!("version_1");
}

// #[update]
// fn version_2() {
//     ic_cdk::println!("version_2");
// }

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade");
}


#[ic_cdk_macros::post_upgrade]
fn post_upgrade(){
    ic_cdk::println!("post_upgrade");
}