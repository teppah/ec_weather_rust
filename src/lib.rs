#[macro_use]
extern crate lazy_static;

use std::error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

lazy_static! {
    static ref ACCEPTED_LANGS: Vec<&'static str> = vec!["en", "fr"];
}

pub struct EcWeatherFeed {
    city_code: String,
    lang: String,
}

pub struct InitError {
    pub message: String
}

impl EcWeatherFeed {
    pub fn new(city_code: String, lang: String) -> Result<EcWeatherFeed, InitError> {
        if !ACCEPTED_LANGS.contains(&lang.as_str()) {
            return Err(
                InitError {
                    message: format!(
                        "Invalid lang supplied: \"{}\". Accepted langs are {}.",
                        lang,
                        ACCEPTED_LANGS.join(", "))
                }
            );
        }
        Ok(EcWeatherFeed {
            city_code,
            lang,
        })
    }

    pub fn query(&self) -> String {
       "Query Data".to_string()
    }
}