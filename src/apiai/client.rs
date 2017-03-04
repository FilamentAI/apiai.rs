use apiai::lang::Language;

use std::collections::HashMap;

use uuid::Uuid;

use serde::ser::{Serialize,Serializer,SerializeStruct};

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* ApiAIClient is used to coordinate calls to the API.ai REST services.
*
* APIAIClient calls the API.AI REST service using hyper. The API version and base url can be
* configured and an access token for calling the API must be provided when creating the struct.
*
*/
pub struct ApiAIClient{
    pub version: String,
    pub access_token: String,
    pub base_url: String,
}


pub static DEFAULT_BASE_URL: &'static str = "https://api.api.ai/v1/";
pub static DEFAULT_VERSION: &'static str = "20150910";


impl Default for ApiAIClient{
    fn default() -> ApiAIClient{
        ApiAIClient{
            access_token: String::new(),
            version: String::from(DEFAULT_VERSION),
            base_url: String::from(DEFAULT_BASE_URL)
        }
    }
}

impl ApiAIClient{

    /**
    * Carry out an API.ai query
    *
    */
    pub fn query(query: ApiRequest) -> Result<ApiResponse, ApiError>{
        Result::Err(ApiError{})
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* Error message return from API.ai
*/
#[derive(Serialize,Deserialize)]
pub struct ApiError{

}

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* Response from API.AI call
*/
#[derive(Serialize,Deserialize)]
pub struct ApiResponse {

    pub id : String,
    pub timestamp : String,
    pub result: ApiResult

}

//////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize,Deserialize)]
pub struct ApiResult {

    pub source : String,
    #[serde(rename = "resolvedQuery")]
    pub resolved_query : String,
    pub action : String,

    /**
    * `true` if the triggered intent has required parameters and not all the required parameter
    * values have been collected `false` if all required parameter values have been collected
    * or if the triggered intent doesn't containt any required parameters.
    */
    #[serde( rename = "actionIncomplete") ]
    pub action_incomplete : bool,
    /**
    * A map of parameters associated with this result
    */
    pub parameters : HashMap<String,String>,
    /**
    * Vector of contexts provided by the current conversation
    */
    pub contexts : Vec<ApiContext>,
    /**
    * Metadata from api.ai
    */
    pub fulfillment : Vec<ApiMessage>,
    /**
    * Metadata from api.ai
    */
    pub metadata : Vec<ApiMetadata>



}
//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* API.ai metadata struct
*/
#[derive(Serialize,Deserialize)]
pub struct ApiMetadata{
    #[serde( rename = "intentId") ]
    pub intent_id: String,
    #[serde( rename = "webhookUsed") ]
    pub webhook_used: String,
    #[serde( rename = "intentName") ]
    pub intent_name: String,
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* API.ai fullfilment structure (part of results)
*/
#[derive(Serialize,Deserialize)]
pub struct ApiFulfillment{
    pub speech: String,
    /**
    * Represents an array of message objects as described in ApiMessage
    *
    */
    pub messages: Vec<ApiMessage>
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/**
* ApiMesages contain various types of message - text, images, buttons and more.
*
*/
#[derive(Deserialize)]
pub enum ApiMessage{
    Text(String),
    Image(String)
}

impl ApiMessage {

    fn message_type(&self) -> i64 {
        match self{
            &ApiMessage::Text(_) => 0,
            &ApiMessage::Image(_) => 3
        }
    }

}

impl Serialize for ApiMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {

        let len = match self {
            &ApiMessage::Text(_) | &ApiMessage::Image(_)  => 2,
        };

        let msgtype : i64 = self.message_type();

        let mut struc = serializer.serialize_struct("ApiMessage", len)?;

        struc.serialize_field("type", &msgtype)?;

        match self{

            &ApiMessage::Text(ref text)  => struc.serialize_field("speech", &text)?,
            &ApiMessage::Image(ref image_url) => struc.serialize_field("imageUrl", &image_url)?
        };

        struc.end()
    }
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
*  An ApiQuery is a simple string newtype that holds the query payload for api.ai requests.
*
* The [api.ai documentation](https://docs.api.ai/docs/query) defines a query as the natural
* language to be processed that may be up to 256 characters in length.
*/
#[derive(Serialize,Deserialize)]
pub struct ApiQuery(pub String);

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* ApiRequest is a structure that encapsulates an api.ai request object
*
*/
#[derive(Serialize,Deserialize)]
pub struct ApiRequest{
    #[serde(skip_serializing_if="Option::is_none",default)]
    pub query: Option<ApiQuery>,
    #[serde(skip_serializing_if="Option::is_none",default)]
    pub event: Option<ApiEvent>,
    #[serde( rename = "sessionId") ]
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
