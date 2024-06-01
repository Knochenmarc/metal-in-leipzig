use std::borrow::Borrow;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::event::{Event, EventType, Location};
use crate::site::Site;
use crate::tools::Http;

pub struct Festivals<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Festivals<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "fest",
                name: "Festivals",
                website: "",
            },
        }
    }
}

impl Site for Festivals<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        let mut inflammen = Event::new(
            "In Flammen Open Air".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 7, 11).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.in-flammen.com/".to_string(),
            Some("https://image.jimcdn.com/app/cms/image/transf/dimension=635x10000:format=jpg/path/sfa7e4f2e650d1c8b/image/i12b8f7add1cd5e4f/version/1715538710/image.jpg".to_string()),
        );
        inflammen.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 7, 13).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        inflammen.evt_type = EventType::Festival;

        let mut rock = Event::new(
            "Rock um zu Helfen".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 10, 11).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.rock-um-zu-helfen.de/".to_string(),
            Some("https://s3-eu-west-1.amazonaws.com/static.csone.dgbrt.de/artifacts/events/466/design.png".to_string()),
        );
        rock.end_date = Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 10, 12).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 00).unwrap(),
        ));
        rock.evt_type = EventType::Festival;

        let mut impericon = Event::new(
            "Impericon Festival".to_string(),
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 3, 30).unwrap(),
                NaiveTime::default(),
            ),
            self.location.borrow(),
            "https://www.impericon.com/de/festival".to_string(),
            Some("https://www.impericon.com/1248x755x90/media/impericon/tickets/impericonfestivals2024/20231219_imp_fest_24_microseite_vo4_bands_fullsize_leipzig.jpg".to_string()),
        );
        impericon.evt_type = EventType::Festival;

        vec![inflammen, rock, impericon]
    }
}
