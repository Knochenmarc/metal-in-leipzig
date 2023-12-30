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
                NaiveDate::from_ymd_opt(2023, 7, 13).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.in-flammen.com/".to_string(),
            Some("https://image.jimcdn.com/app/cms/image/transf/none/path/sfa7e4f2e650d1c8b/backgroundarea/i06ee0d469e3774ec/version/1669229669/image.jpg".to_string()),
        );
        evt.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 7, 15).unwrap(),
            NaiveTime::default(),
        ));
        evt.evt_type = EventType::Festival;

        vec![evt]
    }
}
