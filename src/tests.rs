use apiai::lang::Language;
use std::collections::HashMap;
use std::option::Option;

extern crate serde_json;

/**
* Simple test to make sure that languages serialize predictably into JSON strings
*/
#[test]
fn test_language_json_serialization() {

   let english = Language::English;

   assert!(String::from("\"en\"") == serde_json::to_string(&english).unwrap());
}

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
