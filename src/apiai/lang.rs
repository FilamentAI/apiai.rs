use serde::ser::{Serialize, Serializer};
use serde::de::{Visitor,Deserialize, Deserializer, Error};

use std::fmt;

/**
* Enum defines API.ai supported languages and their text values used on the server
*
*/
#[derive(PartialEq)]
pub enum Language {
    BrazilianPortuguese,
    ChineseCantonese,
    ChineseSimplified,
    ChineseTraditional,
    English,
    Dutch,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Russian,
    Spanish,
    Ukranian,
}

impl Language{

    /**
    * This function can be called by a language enum instance to get the associated string
    * for use inside the api
    */
    pub fn value(&self) -> &'static str {

        match *self {
            Language::BrazilianPortuguese => "pt-BR",
            Language::ChineseCantonese => "zh-HK",
            Language::ChineseSimplified=> "zh-CN",
            Language::ChineseTraditional => "zh-TW",
            Language::English => "en",
            Language::Dutch => "nl",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Spanish => "es",
            Language::Ukranian => "uk"
        }

    }

}

#[derive(Debug)]
struct LanguageVisitor;


impl Visitor for LanguageVisitor {
    type Value = Language;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, value: &str) -> Result<Language, E>
        where E: Error
    {
        match value {
            "pt-BR" => Ok(Language::BrazilianPortuguese),
            "zh-HK" => Ok(Language::ChineseCantonese),
            "zh-CN" => Ok(Language::ChineseSimplified),
            "zh-TW" => Ok(Language::ChineseTraditional),
            "en" => Ok(Language::English),
            "nl" => Ok(Language::Dutch),
            "fr" => Ok(Language::French),
            "de" => Ok(Language::German),
            "it" => Ok(Language::Italian),
            "ja" => Ok(Language::Japanese),
            "ko" => Ok(Language::Korean),
            "pt" => Ok(Language::Portuguese),
            "ru" => Ok(Language::Russian),
            "es" => Ok(Language::Spanish),
            "uk" => Ok(Language::Ukranian),
             _ => Err(E::custom(format!("Language {} was not a supported string.", value).as_str()  ))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Language, E>
        where E: Error
    {
        self.visit_str(value.as_str())
    }

}

// JSON value representation
impl Serialize for Language{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(self.value())
    }
}

impl Default for Language {
    fn default() -> Language { Language::English }
}

impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Deserialize for Language {
    fn deserialize<D>(deserializer: D) -> Result<Language, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_str(LanguageVisitor)
    }
}
