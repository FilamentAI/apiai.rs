// Import all data structures and re-export from api client.
pub use structure::{
        ApiRequest,
        ApiEvent,
        ApiResponse,
        ApiStatus,
        ApiResult,
        ApiMetadata,
        ApiMessage,
        ApiFulfillment,
};

use serde_json;
use hyper::client::{Client, Response};
use hyper::header::{Authorization, Bearer, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std;
use std::io::Read;


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


pub static DEFAULT_BASE_URL: &'static str = "https://api.api.ai/v1";
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
    pub fn query(&self, query: ApiRequest) -> Result<ApiResponse, ApiError>{

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let query_url = self.base_url.clone() + "/query?v=" + self.version.as_str();

        client.post(query_url.as_str())
            .header(Authorization(Bearer {token: self.access_token.clone() }))
            .header(ContentType::json())
            .body(serde_json::to_string(&query).unwrap().as_str())
            .send()
            .map_err(map_hyper_to_api_err)
            .and_then(deserialize_api_response)

    }
}

fn deserialize_api_response(mut response : Response) -> Result<ApiResponse, ApiError>{
    let mut result_buffer = String::new();
    match response.read_to_string(&mut result_buffer) {
        Result::Ok(_) => {
                serde_json::from_str(result_buffer.as_str()).map_err(map_serde_to_api_err)
        },
        Result::Err(err) => Result::Err(map_hyper_to_api_err(err))
    }

}


fn map_hyper_to_api_err<E>(err : E) ->ApiError where E : std::error::Error {
    ApiError::HttpError(String::from(err.description()))
}


fn map_serde_to_api_err(err : serde_json::Error) -> ApiError {

    ApiError::SerializationError(err)
}

//////////////////////////////////////////////////////////////////////////////////////////////////

/**
* Error message return from API.ai
*/
#[derive(Debug)]
pub enum ApiError{
    SerializationError(serde_json::Error),
    HttpError(String)
}
