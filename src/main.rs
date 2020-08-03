use std::io::BufReader;
use std::process;

use clap::{App, Arg};
use xml::EventReader;
use xml::reader::XmlEvent;

use ec_weather_rust::EcWeatherFeed;

#[allow(unused_imports)]
use tokio::prelude::*;

static CITY: &str = "city";
static LANG: &str = "lang";


#[tokio::main]
async fn main() {
    let app = App::new("Environment Canada Weather CLI")
        .version("1.0")
        .about("Get weather from Environment Canada and print it out")
        .arg(Arg::with_name(CITY)
            .short("c")
            .long("city")
            .value_name("CITY")
            .help("Sets the 5-character city code assigned by Environment Canada")
            .required(true))
        .arg(Arg::with_name(LANG)
            .short("l")
            .long("lang")
            .value_name("LANG")
            .help("Sets the language of the data")
            .required(false)
            .default_value("en"));
    let matches = app.get_matches();
    // safe to unwrap since values will always be here
    // lifetime of slices will match the underlying OsStrings, which are held by App
    //      so same lifetime as App
    let city = matches.value_of(CITY).unwrap();
    let lang = matches.value_of(LANG).unwrap();

    let feed = EcWeatherFeed::new(city.to_string(), lang.to_string())
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e.message);
            process::exit(1);
        });
    process(feed).await;
}

async fn process(feed: EcWeatherFeed) {
    let result = feed.query().await;
    let data = match result {
        Ok(data) => {
            data
        }
        Err(e) => {
            if e.status().unwrap().eq(&reqwest::StatusCode::NOT_FOUND) {
                eprintln!("Error: city code not found: {}", feed.city_code);
            } else {
                eprintln!("Error: {}", e);
            }
            return;
        }
    };

    let file = BufReader::new(data.as_bytes());
    let mut parser = EventReader::new(file);

    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                println!("----");
                println!("new tag: {}", name.local_name);
            }
            Ok(XmlEvent::Characters(str)) => {
                println!("{}", str);
            }
            Ok(XmlEvent::EndDocument) => break,
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error parsing document: {}", e);
            }
        }
    }
}
