use apiai::lang::Language;

use std::collections::HashMap;

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
            access_token: String::from(""),
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
    pub fn query() -> Result<ApiResponse, ApiError>{
        Result::Ok(ApiResponse{})
    }
}

#[derive(Serialize,Deserialize)]
pub struct ApiError{

}

#[derive(Serialize,Deserialize)]
pub struct ApiResponse {

}

#[derive(Serialize,Deserialize)]
pub struct ApiContext{
    pub name: String,
    pub parameters: HashMap<String, String>,
    pub lifespan: Option<i32>
}

#[derive(Serialize,Deserialize)]
pub struct ApiEvent{
    pub name: String,
    pub data: Option<HashMap<String,String>>
}

#[derive(Serialize,Deserialize)]
pub struct ApiQuery(pub String);



#[derive(Serialize,Deserialize,Default)]
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
