use apiai::lang::Language;
use std::collections::HashMap;
use std::option::Option;
use apiai::client::{ApiQuery, ApiRequest, ApiEvent};

extern crate serde_json;

/**
* Simple test to make sure that languages serialize predictably into JSON strings
*/
#[test]
fn test_language_json_serialization() {

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

#[test]
fn test_language_json_deserialization() {

   let en_str : &'static str = "{\"lang\":\"en\"}";

   let langmap : LangObject = serde_json::from_str(en_str).unwrap();

   assert!(langmap.lang == Language::English);
}

#[test]
fn test_apiquery_query_json_serialization() {
    let q = ApiQuery::Query(String::from("hello moto"));
    let query_string = "{\"query\":\"hello moto\"}";

    assert!(String::from(query_string) == serde_json::to_string(&q).unwrap());
}

#[test]
fn test_apiquery_event_no_args_json_serialization() {
    let e = ApiQuery::Event(ApiEvent{name: String::from("Welcome"), data: Option::None});
    let event_string = "{\"event\":{\"name\":\"Welcome\",\"data\":null}}";

    assert!(String::from(event_string) == serde_json::to_string(&e).unwrap());
}

#[test]
fn test_apiquery_event_with_args_json_serialization() {
    let mut data = HashMap::new();
    data.insert(String::from("client"), String::from("Slack"));

    let e = ApiQuery::Event(ApiEvent{
        name: String::from("Welcome"),
        data: Option::Some(data)
    });

    let event_string = "{\"event\":{\"name\":\"Welcome\",\"data\":{\"client\":\"Slack\"}}}";


    assert!(String::from(event_string) == serde_json::to_string(&e).unwrap());
}
