#[macro_use]
extern crate lazy_static;

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

#[derive(Debug)]
pub struct EcWeatherFeed {
    pub city_code: String,
    pub lang: String,
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