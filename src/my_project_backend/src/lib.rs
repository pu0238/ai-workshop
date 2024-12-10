use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::HttpHeader;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
async fn translate() -> String {
    let args = CanisterHttpRequestArgument {
        url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: vec![
            HttpHeader {
                name: "Authorization".to_string(),
                value: "Bearer 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".to_string()
            }
        ],
        body: Some(r#"{"inputs": "My name is Sarah and I live in London"}"#.into()),
        transform: None,
    };
    let cycles = 2_000_000_000u128;
    let res = http_request(args, cycles).await.expect("Failed to call service").0;
    ic_cdk::println!("{:?}", res);
    let body_string = String::from_utf8(res.body).expect("Failed to read body");
    ic_cdk::println!("{:?}", body_string);
    body_string
} 