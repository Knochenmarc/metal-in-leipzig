use std::borrow::Borrow;
use std::str::FromStr;

use chrono::NaiveTime;
use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_short_date;
use crate::tools::HTTP;

pub struct ConneIsland {
    location: Location,
}

impl ConneIsland {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "ci".to_string(),
                name: "Conne Island".to_string(),
                website: "https://conne-island.de".to_string(),
            },
        }
    }
}

impl Site for ConneIsland {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();
        let rss = http.get_rss("https://www.conne-island.de/rss.php?genre=Metal");
        for item in rss.items {
            let title = item.title.unwrap();
            result.push(Event::new(
                title.as_str()[12..].to_string(),
                parse_short_date(title.as_str()[..10].borrow()).and_hms(0, 0, 0),
                self.location.borrow(),
                item.link.unwrap().replace(r"http://", r"https://"),
                Option::None,
            ));
        }

        result
    }
}
