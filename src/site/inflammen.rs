use std::borrow::Borrow;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::event::{Event, EventType, Location};
use crate::site::Site;
use crate::tools::Http;

pub struct InFlammen<'l> {
    location: Location<'l, 'l, 'l>,
}

impl InFlammen<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "if",
                name: "In Flammen Open Air",
                website: "https://www.in-flammen.com/",
            },
        }
    }
}

impl Site for InFlammen<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        let mut evt = Event::new(
            "In Flammen Open Air".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 7, 11).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.in-flammen.com/".to_string(),
            Some("https://image.jimcdn.com/app/cms/image/transf/none/path/sfa7e4f2e650d1c8b/backgroundarea/i6f52df9656bc8d1b/version/1693393571/image.jpg".to_string()),
        );
        evt.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 13).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        evt.evt_type = EventType::Festival;

        vec![evt]
    }
}
