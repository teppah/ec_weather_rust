extern crate clap;

use std::borrow::Borrow;
use std::process;

use clap::{App, Arg};
use ec_weather_rust::EcWeatherFeed;

static CITY: &str = "city";
static LANG: &str = "lang";

fn main() {
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
    println!("lang: {}, city: {}", lang, city);

    let feed = EcWeatherFeed::new(city.to_string(), lang.to_string()).unwrap_or_else(|e| {
        eprintln!("Error: {}", e.message);
        process::exit(1);
    });


    let feed = feed.query();
    println!("{}", feed);
}