use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpMethod};
use ic_cdk::{update};

use candid::{candid_method, Principal};


#[update]
#[candid_method(update)]
fn one_shot_outcall() -> () {

    let arg = CanisterHttpRequestArgument {
        url: "https://webhook.site/fcee8cb5-ae12-40d2-95d2-05a899d74092".to_string(),
        method: HttpMethod::POST,
        body: None,
        max_response_bytes: Some(0),
        transform: None,
        headers: [].to_vec(),
    };

    let cycles = http_request_required_cycles(&arg);
    let _ = ic_cdk::api::call::notify_with_payment128(
        Principal::management_canister(),
        "http_request",
        (arg,),
        cycles,
    );
}
    
fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // TODO: this formula should be documented somewhere
    // 12 is "http_request".len().
    400_000_000u128 + 100_000u128 * (arg_raw.len() as u128 + 12 + max_response_bytes)
}
