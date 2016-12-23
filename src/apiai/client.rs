use apiai::lang::Language;

use std::collections::HashMap;

use serde::{Serialize,Serializer};

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
pub enum ApiQuery{
    #[serde(rename = "query")]
    Query(String),
    #[serde(rename = "event")]
    Event(ApiEvent)
}

#[derive(Deserialize)]
pub struct ApiRequest{
    pub query: ApiQuery,
    pub session_id: String,
    pub lang: Language,
    pub contexts: Vec<ApiContext>
}


impl Serialize for ApiRequest {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer {


        let mut state = try!(serializer.serialize_struct("",4));

        match self.query {
            // if query then serialize only the string value
            ApiQuery::Query(ref x) => {
                try!(serializer.serialize_struct_elt(&mut state, "query", x));
            }

            ApiQuery::Event(ref evt) =>  {
                try!(serializer.serialize_struct_elt(&mut state, "event", evt));
            }
        }

        // serialize session id
        try!(serializer.serialize_struct_elt(&mut state, "sessionId", &self.session_id));

        //serialize language
        try!(serializer.serialize_struct_elt(&mut state, "lang", &self.lang));

        //serialize contexts if any
        try!(serializer.serialize_struct_elt(&mut state, "contexts", &self.contexts));

        serializer.serialize_struct_end(state)

    }
}
