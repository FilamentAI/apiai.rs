// Import all data structures and re-export from api client.
pub use apiai::structure::{
        ApiRequest,
        ApiQuery,
        ApiEvent,
        ApiResponse,
        ApiStatus,
        ApiResult,
        ApiMetadata,
        ApiMessage,
        ApiFulfillment,
};


//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* ApiAIClient is used to coordinate calls to the API.ai REST services.
*
* APIAIClient calls the API.AI REST service using hyper. The API version and base url can be
* configured and an access token for calling the API must be provided when creating the struct.
*
*/
#[derive(Serialize,Deserialize)]
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
