use apiai::lang::Language;

use std::collections::HashMap;

use uuid::Uuid;

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* ApiAIClient is used to coordinate calls to the API.ai REST services.
*
* APIAIClient calls the API.AI REST service using hyper. The API version and base url can be
* configured and an access token for calling the API must be provided when creating the struct.
*
*/
pub struct ApiAIClient{
    version: String,
    access_token: String,
    base_url: String,
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
        Result::Ok(ApiResponse{})
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
