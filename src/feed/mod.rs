lazy_static! {
    static ref ACCEPTED_LANGS: Vec<&'static str> = vec!["en", "fr"];
}
pub mod parsed;

#[derive(Debug)]
pub struct EcWeatherFeed {
    pub city_code: String,
    pub lang: String,
}

#[derive(Debug)]
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

    pub async fn query(&self) -> Result<String, reqwest::Error> {
        let lang_char = self.lang.chars().nth(0).unwrap();
        let url = format!("https://weather.gc.ca/rss/city/{city}_{lang}.xml", city = self.city_code, lang = lang_char);
        let response = reqwest::get(url.as_str())
            .await?;
        // if not a 200 code, will Err
        match response.error_for_status() {
            Ok(res) => {
                let text = res.text().await?;
                Ok(text)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn correct_init() {
        static CITY: &str = "city-code";
        static LANG: &str = "en";
        let feed = EcWeatherFeed::new(CITY.to_string(), LANG.to_string());
        let feed = feed.unwrap_or_else(|e| {
            panic!("Did not correctly initialize and threw error: {:?}", e);
        });
        assert_eq!(feed.city_code, CITY.to_string());
        assert_eq!(feed.lang, LANG.to_string());
    }

    #[test]
    fn bad_lang() {
        let bad = EcWeatherFeed::new("any".to_string(), "bad".to_string());
        assert!(bad.is_err());
        let err = bad.unwrap_err();
        assert!(err.message.contains("bad"));
    }
}