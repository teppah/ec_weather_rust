use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::BufReader;

use xml::EventReader;
use xml::reader::XmlEvent;

lazy_static! {
   pub static ref CURRENT_CONDITIONS: &'static str = "Current Conditions";
   pub static ref FORECAST: &'static str = "Weather Forecasts";
}

#[derive(Debug)]
pub struct ParsedFeed {
    pub title: String,
    pub feed_link: String,
    pub page_link: String,
    pub last_updated: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub category: String,
    pub summary: String,
}

impl Entry {
    pub fn new(title: String, category: String, summary: String) -> Entry {
        Entry { title, category, summary }
    }
}

impl ParsedFeed {
    fn new_empty() -> ParsedFeed {
        ParsedFeed {
            title: String::new(),
            feed_link: String::new(),
            page_link: String::new(),
            last_updated: String::new(),
            entries: Vec::with_capacity(15)
            ,
        }
    }
}

impl Display for ParsedFeed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Feed")
    }
}

const TITLE: &'static str = "title";
const LINK: &'static str = "link";
const SELF_LINK: &'static str = "self";
const RELATED_LINK: &'static str = "related";
const UPDATE_DATE: &'static str = "updated";

const ENTRY: &'static str = "entry";
const CATEGORY: &'static str = "category";
const SUMMARY: &'static str = "summary";


pub fn parse_feed_from_str(feed: &str) -> ParsedFeed {
    let feed = feed.as_bytes();
    let reader = BufReader::new(feed);
    let mut parser = EventReader::new(reader);

    let mut parsed = ParsedFeed::new_empty();
    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                let local = name.local_name;
                match local.as_str() {
                    ENTRY => {
                        println!("entry");
                    }
                    LINK => {}
                    TITLE => {}
                    UPDATE_DATE => {}
                    _ => ()
                }
            }
            Ok(XmlEvent::EndDocument) => break,
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error parsing document: {}", e);
            }
        }
    };
    parsed
}
