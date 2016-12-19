use apiai::lang::Language;
use serde;

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
}

pub struct ApiQuery{
    pub query: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub lang: Language
}
