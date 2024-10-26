use std::borrow::Borrow;

use crate::site::google_calendar::fetch_calendar_events;
use crate::site::tixforgigs::fetch_tixforgigs_events;
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
        let mut calendar_events = fetch_calendar_events(http, "AIzaSyBNlYH01_9Hc5S1J9vuFmu2nUqBZJNAXxs", "82622f9c315eef480e39200aaf98aff4c60a87db0d2c33e14a0bbb2a71b97b8c@group.calendar.google.com", self.get_location());
        let tixforgigs_events = fetch_tixforgigs_events(http, "3707", self.get_location());

        for tixforgigs_event in tixforgigs_events {
            let mut found_index: Option<usize> = None;
            for (index, calendar_event) in calendar_events.iter().enumerate() {
                if calendar_event.start_date.date() == tixforgigs_event.start_date.date() {
                    found_index = Some(index);
                    break;
                }
            }
            if let Some(index) = found_index {
                calendar_events.swap_remove(index);
            }
            calendar_events.push(tixforgigs_event)
        }

        calendar_events
    }
}
