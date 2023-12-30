use std::borrow::Borrow;

use crate::site::google_calendar::fetch_calendar_events;
use crate::{Event, Http, Location, Site};

pub(crate) struct Soltmann<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Soltmann<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "sm",
                name: "Soltmann",
                website: "https://soltmann.club/",
            },
        }
    }
}

impl Site for Soltmann<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        fetch_calendar_events(http, "ODI2MjJmOWMzMTVlZWY0ODBlMzkyMDBhYWY5OGFmZjRjNjBhODdkYjBkMmMzM2UxNGEwYmJiMmE3MWI5N2I4Y0Bncm91cC5jYWxlbmRhci5nb29nbGUuY29t", self.get_location())
    }
}
