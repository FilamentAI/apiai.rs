use apiai::lang::Language;

use std::collections::HashMap;

use serde::Serialize;

pub struct ApiAIClient{
    version: String,
    access_token: String,
    base_url: String,
}

pub static DEFAULT_BASE_URL: &'static str = "https://api.api.ai/v1/";
pub static DEFAULT_VERSION: &'static str = "20150910";

impl ApiAIClient{

    pub fn new(access_token: String, version: String, base_url: String) -> ApiAIClient{
        ApiAIClient{ version: version, access_token: access_token, base_url: base_url}
    }

    /**
    * Carry out an API.ai query
    *
    */
    pub fn query() -> ApiResponse{
        ApiResponse{}
    }
}


#[derive(Serialize)]
pub struct ApiResponse {

}

#[derive(Serialize)]
pub struct ApiContext{
    pub name: String,
    pub parameters: HashMap<String, String>,
    pub lifespan: Option<i32>
}

#[derive(Serialize)]
pub struct ApiEvent{
    pub name: String,
    pub data: Option<HashMap<String,String>>
}

#[derive(Serialize)]
pub enum ApiQuery{
    #[serde(rename = "query")]
    Query(String),
    #[serde(rename = "event")]
    Event(ApiEvent)
}

#[derive(Serialize)]
pub struct ApiRequest{
    pub query: ApiQuery,

    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub lang: Language,
    pub contexts: Vec<ApiContext>
}
