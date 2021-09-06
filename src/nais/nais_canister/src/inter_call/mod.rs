pub mod types;

use ic_cdk::api;
use candid::{CandidType, Principal};
use nft::{NFTPayload, Property, NftEgg, Value};
use serde::Deserialize;
use types::*;

pub async fn create_canister_call(args: CreateCanisterArgs) -> Result<CreateResult, String> {
    #[derive(CandidType)]
    struct In {
        settings: Option<CanisterSettings>,
    }
    let in_arg = In {
        settings: Some(args.settings),
    };

    let (create_result,): (CreateResult,) = match api::call::call_with_payment(
        Principal::management_canister(),
        "create_canister",
        (in_arg,),
        args.cycles,
    ).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during create_canister call: {}: {}",
                code as u8, msg
            ))
        }
    };

    Ok(create_result)
}

pub async fn start_call(args: StartStopArgs) -> Result<(), String> {
    match api::call::call(Principal::management_canister(), "start_canister", (args,)).await {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Ok(())
}
pub async fn stop_call(args: StartStopArgs) -> Result<(), String> {
    match api::call::call(Principal::management_canister(), "stop_canister", (args,)).await {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Ok(())
}

pub async fn install_canister(
    canister_id: &Principal,
    wasm_module: Vec<u8>,
    args: Vec<u8>,
) -> Result<(), String> {
    // Install Wasm
    #[derive(CandidType, Deserialize)]
    enum InstallMode {
        #[serde(rename = "install")]
        Install,
        #[serde(rename = "reinstall")]
        Reinstall,
        #[serde(rename = "upgrade")]
        Upgrade,
    }

    #[derive(CandidType, Deserialize)]
    struct CanisterInstall {
        mode: InstallMode,
        canister_id: Principal,
        #[serde(with = "serde_bytes")]
        wasm_module: Vec<u8>,
        #[serde(with = "serde_bytes")]
        arg: Vec<u8>,
    }

    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id: canister_id.clone(),
        wasm_module: wasm_module.clone(),
        arg: args,
    };

    match api::call::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    ).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during install_code call: {}: {}",
                code as u8, msg
            ))
        }
    };
    Ok(())
}

// pub async fn update_settings_call(args: UpdateSettingsArgs) -> Result<(), String> {
//     match api::call::call(Principal::management_canister(), "update_settings", (args,)).await {
//         Ok(x) => x,
//         Err((code, msg)) => {
//             return Err(format!(
//                 "An error happened during the call: {}: {}",
//                 code as u8, msg
//             ))
//         }
//     };
//     Ok(())
// }

pub async fn mint_citizen_nft(nft_canister: &VisaNFTCanisterId, citizen: Principal) -> Result<String, String> {
    let egg = NftEgg {
        payload: NFTPayload::Payload(vec![0x00]),
        content_type: Default::default(),
        owner: citizen,
        properties: vec![Property{
            name : String::from("citizenship"),
            value : Value::Empty,
            immutable : false
        }],
        is_private: true
    };
    
    let (nft_id,): (String,) = match api::call::call(
        nft_canister.clone(),
        "mint", 
        (egg,)
    ).await 
    {
        Ok(x) => x,
        Err((code, msg)) => {
            return Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    };

    Ok(nft_id)
}

