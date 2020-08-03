#[macro_use]
extern crate lazy_static;

pub use feed::EcWeatherFeed;
pub use feed::InitError;
pub use feed::parsed::ParsedFeed;
pub use feed::parsed::Entry;
pub use feed::parsed::parse_feed_from_str;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod feed;
