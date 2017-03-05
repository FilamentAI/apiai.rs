use uuid::Uuid;
use apiai::lang::Language;
use std::collections::HashMap;

/**
* Response from API.AI call
*/
#[derive(Serialize,Deserialize)]
pub struct ApiResponse {
    pub id : String,
    pub timestamp : String,
    #[serde(default="Language::default")]
    pub lang : Language,
    pub result: ApiResult,

    /**
    * The status object is returned with every request and indicates
    * if the request was successful. If it is not successful, error information is included.
    *
    */
    pub status : ApiStatus,

    /**
    * Session ID
    *
    */
    #[serde( rename = "sessionId" ) ]
    pub session_id : String

}


//////////////////////////////////////////////////////////////////////////////////////////////////
/**
* Result object encapsulates API result including actions, intent name (if applicable) etc.
*
*/
#[derive(Serialize,Deserialize)]
pub struct ApiResult {

    /**
    * Source of the answer. Could be "agent" if the response was retrieved from the agent.
    *
    */
    pub source : String,
    /**
    * The query that was used to produce this result.
    *
    */
    #[serde(rename = "resolvedQuery")]
    pub resolved_query : String,
    /**
    * An action to take. `Example: turn on`
    *
    */
    pub action : String,

    /**
    * `true` if the triggered intent has required parameters and not all the required parameter
    * values have been collected `false` if all required parameter values have been collected
    * or if the triggered intent doesn't containt any required parameters.
    */
    #[serde( rename = "actionIncomplete", default="default_bool") ]
    pub action_incomplete : bool,
    /**
    * A map of parameters associated with this result
    */
    pub parameters : HashMap<String,String>,
    /**
    * Vector of contexts provided by the current conversation
    */
    #[serde(default="Vec::default")]
    pub contexts : Vec<ApiContext>,

    /**
    * Metadata from api.ai
    */
    pub metadata : ApiMetadata,

    /**
    * Metadata from api.ai
    */
    pub fulfillment : ApiFulfillment,

    /**
    * Matching score for the intent. (Between 1 and 0)
    *
    */
    pub score : f32,


}

fn default_bool() -> bool {false}

//////////////////////////////////////////////////////////////////////////////////////////////////
/**
* API Status encapsulates the api http status - usually 200 if all is well
*
*/
#[derive(Serialize,Deserialize)]
pub struct ApiStatus{
    pub code: i32,
    #[serde( rename = "errorType") ]
    pub error_type: String,
    #[serde( rename = "errorDetails", skip_serializing_if = "Option::is_none") ]
    pub error_details: Option<String>,
}

/**
* API.ai metadata struct
*
*/
#[derive(Serialize,Deserialize)]
pub struct ApiMetadata{
    #[serde( rename = "intentId") ]
    pub intent_id: Option<String>,
    #[serde( rename = "webhookUsed", default="default_false_string") ]
    pub webhook_used: String,
    #[serde( rename = "webhookForSlotFillingUsed", default="default_false_string") ]
    pub webhook_slotfilling_used: String,
    #[serde( rename = "intentName", skip_serializing_if = "Option::is_none") ]
    pub intent_name: Option<String>,
}

fn default_false_string() -> String{ String::from("false") }

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* API.ai fullfilment structure (part of results)
*/
#[derive(Serialize,Deserialize)]
pub struct ApiFulfillment{
    /**
    * The speech to be sent back to the user
    *
    */
    pub speech: String,
    /**
    * Represents an array of message objects as described in ApiMessage
    *
    */
    #[serde(skip_serializing_if = "Option::is_none") ]
    pub messages: Option<Vec<ApiMessage>>
}


//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* API.ai context object representation.
*/
#[derive(Serialize,Deserialize)]
pub struct ApiContext{
    pub name: String,
    pub parameters: HashMap<String, String>,
    pub lifespan: Option<i32>
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* An ApiEvent has a name and may have a list of parameters under the label 'data'.
*/
#[derive(Serialize,Deserialize)]
pub struct ApiEvent{
    pub name: String,
    pub data: Option<HashMap<String,String>>
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* ApiRequest is a structure that encapsulates an api.ai request object
*
*/
#[derive(Serialize,Deserialize)]
pub struct ApiRequest{
    /**
    *  queryis a simple string that holds the query payload for api.ai requests.
    *
    * The [api.ai documentation](https://docs.api.ai/docs/query) defines a query as the natural
    * language to be processed that may be up to 256 characters in length.
    */
    #[serde(skip_serializing_if="Option::is_none",default)]
    pub query: Option<String>,
    #[serde(skip_serializing_if="Option::is_none",default)]
    pub event: Option<ApiEvent>,
    #[serde( rename = "sessionId", skip_serializing_if="String::is_empty", default) ]
    pub session_id: String,
    pub lang: Language,
    pub contexts: Vec<ApiContext>
}

/**
* Default for ApiRequest automatically generates a sessionId if one is not provided.
*
*/
impl Default for ApiRequest {

    fn default() -> ApiRequest{
        ApiRequest{
            query: Option::None,
            event: Option::None,
            session_id: Uuid::new_v4().hyphenated().to_string(),
            lang: Language::English,
            contexts: Vec::new()
        }
    }

}
//////////////////////////////////////////////////////////////////////////////////////////////////
/**
* ApiMesages contain various types of message - text, images, buttons and more.
*
*/
#[derive(Serialize,Deserialize)]
#[serde(untagged)]
pub enum ApiMessage{

    Text {
        speech: String,
        #[serde(rename="type")]
        message_type: i32
    },
    Image {
        #[serde(rename="imageUrl")]
        image_url: String,
        #[serde(rename="type")]
        message_type: i32
    }
}

impl ApiMessage {

    pub fn new_text(speech: String) -> ApiMessage {
        ApiMessage::Text{speech: speech, message_type: 0}
    }

    pub fn new_image(image_url: String) -> ApiMessage {
        ApiMessage::Image{image_url: image_url, message_type: 3}
    }

}
