use apiai::lang::Language;
use std::collections::HashMap;
use std::option::Option;
use apiai::client::{
    ApiRequest,
    ApiEvent,
    ApiResponse,
    ApiFulfillment,
    ApiStatus,
    ApiMetadata,
    ApiResult,
    ApiMessage
};

use serde_json;

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
* Test that deserializing an event with no args works as expected
*
*/
#[test]
fn test_serialize_apirequest_query(){

    let query_string = r#"{"query":"hello moto","sessionId":"12345","lang":"en","contexts":[]}"#;

    let req = ApiRequest{
        query: Option::Some(String::from("hello moto")),
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


/**
* Test that we are able to serialize a simple text apimessage
*
*/
#[test]
fn test_serialize_apimessage_text(){

    let apimessage = ApiMessage::new_text(String::from("Hello"));

    let msg_string = r#"{"speech":"Hello","type":0}"#;

    assert_eq!(msg_string, serde_json::to_string(&apimessage).unwrap());
}

/**
* Test that we are able to deserialize the example text request json from the api.ai website.
*
*/
#[test]
fn test_deserialize_apimessage_text(){

    let msg_string = r#"{"type":0,"speech":"Hello"}"#;
    let msg : ApiMessage = serde_json::from_str(&msg_string).unwrap();

    match msg {
        ApiMessage::Text{speech, message_type} => {
            assert_eq!(speech, String::from("Hello"));
            assert_eq!(message_type, 0);
        },

        _ => panic!("Type is suppsed to be text")
    };

}

#[test]
fn test_deserialize_api_response(){
    let json_string = r#"{
  "id": "b340a1f7-abee-4e13-9bdd-5e8938a48b7d",
  "timestamp": "2017-02-09T15:38:26.548Z",
  "lang": "en",
  "result": {
    "source": "agent",
    "resolvedQuery": "my name is Sam and I live in Paris",
    "action": "greetings",
    "actionIncomplete": false,
    "parameters": {},
    "contexts": [],
    "metadata": {
      "intentId": "9f41ef7c-82fa-42a7-9a30-49a93e2c14d0",
      "webhookUsed": "false",
      "webhookForSlotFillingUsed": "false",
      "intentName": "greetings"
    },
    "fulfillment": {
      "speech": "Hi Sam! Nice to meet you!",
      "messages": [
        {
          "type": 0,
          "speech": "Hi Sam! Nice to meet you!"
        }
      ]
    },
    "score": 1
  },
  "status": {
    "code": 200,
    "errorType": "success"
  },
  "sessionId": "4b6a6779-b8ea-4094-b2ed-a302ba201815"
}"#;

    let msg : ApiResponse = serde_json::from_str(&json_string).unwrap();

    assert_eq!(msg.status.code, 200);
    assert_eq!(msg.session_id, String::from("4b6a6779-b8ea-4094-b2ed-a302ba201815"));
    assert_eq!(msg.result.score, 1.0);
    assert_eq!(msg.result.fulfillment.speech, String::from("Hi Sam! Nice to meet you!"));

}


#[test]
fn test_serialize_api_response(){
    let status = ApiStatus{
        code: 200,
        error_type: String::from("success"),
        error_details: Option::None
    };

    let msg = ApiMessage::new_text(String::from("Hi Sam! Nice to meet you!"));

    let fulfillment = ApiFulfillment {
        speech: String::from("Hi Sam! Nice to meet you!"),
        messages: Option::Some(vec!(msg))
    };

    let metadata = ApiMetadata{
        intent_id: Option::Some(String::from("9f41ef7c-82fa-42a7-9a30-49a93e2c14d0")),
        webhook_used: String::from("false"),
        webhook_slotfilling_used: String::from("false"),
        intent_name: Option::Some(String::from("greetings"))
    };

    let result = ApiResult{
        source: String::from("agent"),
        resolved_query: String::from("My name is Sam and I live in Paris"),
        action: String::from("greetings"),
        action_incomplete: false,
        parameters: HashMap::new(),
        contexts: Vec::new(),
        metadata: metadata,
        fulfillment: fulfillment,
        score: 1.0,
    };

    let response = ApiResponse{
        id: String::from("b340a1f7-abee-4e13-9bdd-5e8938a48b7d"),
        timestamp: String::from("2017-02-09T15:38:26.548Z"),
        lang: Language::English,
        result: result,
        status: status,
        session_id: String::from("4b6a6779-b8ea-4094-b2ed-a302ba201815")
    };

    let ideal_output = r#"{"id":"b340a1f7-abee-4e13-9bdd-5e8938a48b7d","timestamp":"2017-02-09T15:38:26.548Z","lang":"en","result":{"source":"agent","resolvedQuery":"My name is Sam and I live in Paris","action":"greetings","actionIncomplete":false,"parameters":{},"contexts":[],"metadata":{"intentId":"9f41ef7c-82fa-42a7-9a30-49a93e2c14d0","webhookUsed":"false","webhookForSlotFillingUsed":"false","intentName":"greetings"},"fulfillment":{"speech":"Hi Sam! Nice to meet you!","messages":[{"speech":"Hi Sam! Nice to meet you!","type":0}]},"score":1.0},"status":{"code":200,"errorType":"success"},"sessionId":"4b6a6779-b8ea-4094-b2ed-a302ba201815"}"#;

    //println!("{}", serde_json::to_string(&response).unwrap());
    assert_eq!(ideal_output, serde_json::to_string(&response).unwrap())
}
