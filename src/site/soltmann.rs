use std::borrow::Borrow;

use crate::site::facebook::fetch_facebook_events;
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

fn merge_lists<'a>(source: Vec<Event<'a>>, mut target: Vec<Event<'a>>) -> Vec<Event<'a>> {
    for source_event in source {
        let mut found_index: Option<usize> = None;
        for (index, target_event) in target.iter().enumerate() {
            if target_event.start_date.date() == source_event.start_date.date() {
                found_index = Some(index);
                break;
            }
        }
        if let Some(index) = found_index {
            target.swap_remove(index);
        }
        target.push(source_event);
    }

    target
}

impl Site for Soltmann<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut calendar_events = fetch_calendar_events(http, "AIzaSyBNlYH01_9Hc5S1J9vuFmu2nUqBZJNAXxs", "82622f9c315eef480e39200aaf98aff4c60a87db0d2c33e14a0bbb2a71b97b8c@group.calendar.google.com", self.get_location());

        let metalheadz_events =
            fetch_facebook_events(http, self.location.borrow(), "MetalheadzEvents");
        let metalheadz_events = metalheadz_events
            .into_iter()
            .filter(|event| event.description.as_ref().unwrap().contains("Soltmann"))
            .collect();
        calendar_events = merge_lists(metalheadz_events, calendar_events);

        let tixforgigs_events = fetch_tixforgigs_events(http, "3707", self.get_location());
        calendar_events = merge_lists(tixforgigs_events, calendar_events);

        calendar_events
    }
}
