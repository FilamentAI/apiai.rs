use client::{ApiAIClient,ApiRequest};
use lang::Language;

#[test]
fn test_send_api_query() {

    let my_token = String::from("ce2f54f8eb444d74af85f89e30ef2fd3");

    let client = ApiAIClient{
        access_token: my_token,
        ..Default::default()
    };

    let req = ApiRequest{
        query: Option::Some(String::from("Hello!")),
        session_id: String::from("1234"),
        lang: Language::English,
        contexts: Vec::new(),
        ..Default::default()
    };

    let response = client.query(req).unwrap();
    assert_eq!(response.result.action, String::from("smalltalk.greetings"))
}
