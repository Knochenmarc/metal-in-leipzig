use std::borrow::Borrow;

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_short_date;
use crate::tools::HTTP;

pub struct ConneIsland<'h> {
    location: Location,
    insecure_http: &'h HTTP,
}

impl<'a> ConneIsland<'a> {
    pub(crate) fn new(insecure_http: &'a HTTP) -> Self {
        Self {
            location: Location {
                slug: "ci".to_string(),
                name: "Conne Island".to_string(),
                website: "https://conne-island.de".to_string(),
            },
            insecure_http,
        }
    }
}

impl<'a> Site for ConneIsland<'a> {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self, http: &HTTP) -> Vec<Event> {
        let mut result = Vec::new();
        let rss = self
            .insecure_http
            .get_rss("https://conne-island.de/rss.php?genre=Metal");
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
