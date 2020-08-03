lazy_static! {
   pub static ref CURRENT_CONDITIONS: &'static str = "Current Conditions";
   pub static ref FORECAST: &'static str = "Weather Forecasts";
}

pub struct ParsedFeed {
    title: String,
    feed_link: String,
    page_link: String,
    last_updated: String,
    entries: Vec<Entry>,
}

pub struct Entry {
    title: String,
    category: String,
    summary: String,
}