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
        arg: ic_cdk::api::id().as_slice().to_vec(),
    };
    ic_cdk::notify::<_>(Principal::management_canister(), "install_code", (install_arg,)).unwrap();
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