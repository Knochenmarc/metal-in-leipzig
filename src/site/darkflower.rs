use std::borrow::Borrow;
use std::env::var;

use crate::event::{Event, Location};
use crate::site::facebook::fetch_facebook_events;
use crate::site::Site;
use crate::tools::Http;

pub struct Darkflower<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Darkflower<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "df",
                name: "Darkflower",
                website: "https://darkflower.club/",
            },
        }
    }
}

impl Site for Darkflower<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        fetch_facebook_events(
            http,
            self.get_location(),
            100064530226536,
            var("ML_DF_KEY").unwrap(),
        )
        .into_iter()
        .filter(|event| {
            event
                .description
                .clone()
                .unwrap_or_default()
                .contains("Metal")
        })
        .collect()
    }
}
