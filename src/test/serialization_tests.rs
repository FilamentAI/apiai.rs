use apiai::lang::Language;
use std::collections::HashMap;
use std::option::Option;
use apiai::client::{ApiQuery, ApiRequest, ApiEvent};

extern crate serde_json;

/**
* Simple test to make sure that languages serialize predictably into JSON strings
*/
#[test]
fn test_serialize_language_json() {

   let english = Language::English;

   assert!(String::from("\"en\"") == serde_json::to_string(&english).unwrap());
}

/**
* Dummy object that contains only a Language object to test serialization
*/
#[derive(Deserialize)]
struct LangObject{
    pub lang: Language
}

/**
* Test that deserializing a language from its locale string (e.g. "en") works ok.
*
*/
#[test]
fn test_deserialize_language() {

   let en_str : &'static str = "{\"lang\":\"en\"}";

   let langmap : LangObject = serde_json::from_str(en_str).unwrap();

   assert_eq!(langmap.lang, Language::English);
}

/**
* Test that creating an Api Query object works as expected
*
*/
#[test]
fn test_serialize_apiquery_json() {
    let q = ApiQuery(String::from("hello moto"));
    let query_string = "\"hello moto\"";

    assert_eq!(String::from(query_string), serde_json::to_string(&q).unwrap());
}

/**
* Test that creating an Api Event object with no 'data' args works as expected
*
*/
#[test]
fn test_serialize_event_no_args() {
    let e = ApiEvent{name: String::from("Welcome"), data: Option::None};
    let event_string = "{\"name\":\"Welcome\",\"data\":null}";

    assert_eq!(String::from(event_string),serde_json::to_string(&e).unwrap());
}

/**
* Test that creating an Api Event object with 'data' args works as expected
*
*/
#[test]
fn test_serialize_event_with_args_json() {
    let mut data = HashMap::new();
    data.insert(String::from("client"), String::from("Slack"));

    let e = ApiEvent{
        name: String::from("Welcome"),
        data: Option::Some(data)
    };

    let event_string = "{\"name\":\"Welcome\",\"data\":{\"client\":\"Slack\"}}";


    assert_eq!(String::from(event_string), serde_json::to_string(&e).unwrap());
}


/**
* Test that  deserializing an event with args works as expected
*
*/
#[test]
fn test_deserialize_apiquery_event_with_args() {

    let event_string = "{\"name\":\"Welcome\",\"data\":{\"client\":\"Slack\"}}";

    let evt : ApiEvent = serde_json::from_str(event_string).unwrap();

    assert_eq!(evt.name, String::from("Welcome"));

    assert_eq!(evt.data.unwrap().get("client").unwrap(), "Slack");
}

/**
* Test that  deserializing an event with no args works as expected
*
*/
#[test]
fn test_deserialize_apiquery_event_without_args() {

    let event_string = "{\"name\":\"Welcome\"}";

    let evt : ApiEvent = serde_json::from_str(event_string).unwrap();

    assert_eq!(evt.name, String::from("Welcome"));

    assert!(evt.data.is_none());


}

/**
* Test that  deserializing an event with no args works as expected
*
*/
#[test]
fn test_deserialize_apiquery_query() {

    let query_string = "\"hello moto\"";

    let query : ApiQuery = serde_json::from_str(query_string).unwrap();

    assert!(query.0 == String::from("hello moto"));
}


/**
* Test that deserializing an event with no args works as expected
*
*/
#[test]
fn test_serialize_apirequest_query(){

    let query_string = r#"{"query":"hello moto","sessionId":"12345","lang":"en","contexts":[]}"#;

    let q = ApiQuery(String::from("hello moto"));

    let req = ApiRequest{
        query: Option::Some(q),
        session_id: String::from("12345"),
        lang: Language::English,
        contexts: Vec::new(),
        ..Default::default()
    };

    assert_eq!(query_string, serde_json::to_string(&req).unwrap());
}


/**
* Test that deserializing an event with no args works as expected
*
*/
#[test]
fn test_serialize_apirequest_event_no_data(){

    let query_string = r#"{"event":{"name":"Welcome","data":null},"sessionId":"12345","lang":"en","contexts":[]}"#;

    let e = ApiEvent{
        name: String::from("Welcome"),
        data: Option::None
    };

    let req = ApiRequest{
        event: Option::Some(e),
        session_id: String::from("12345"),
        lang: Language::English,
        contexts: Vec::new(),
        ..Default::default()
    };

    assert_eq!(query_string, serde_json::to_string(&req).unwrap());
}


/**
* Test that deserializing an event with no args works as expected
*
*/
#[test]
fn test_deserialize_apirequest_event_no_data(){

    let query_string = r#"{"event":{"name":"Welcome","data":null},"sessionId":"12345","lang":"en","contexts":[]}"#;

    let req : ApiRequest = serde_json::from_str(query_string).unwrap();

    let event = req.event.unwrap();

    assert_eq!(event.name, "Welcome");
    assert!(event.data.is_none());
}

/**
* Test that deserializing an event with some data args works as expected
*
*/
#[test]
fn test_deserialize_apirequest_event_with_data(){

    let query_string = r#"{"event":{"name":"Welcome","data":{"test":"arg1"}},"sessionId":"12345","lang":"en","contexts":[]}"#;

    let req : ApiRequest = serde_json::from_str(query_string).unwrap();

    let event = req.event.unwrap();

    assert_eq!(event.name, "Welcome");
    let data = event.data.unwrap();

    assert_eq!(data.get("test").unwrap(), "arg1");
}
