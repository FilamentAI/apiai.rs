use serde::ser::{Serialize,Serializer,SerializeStruct};

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
