use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::{get_today, parse_iso_datetime};
use crate::tools::Http;
use chrono::Months;
use std::borrow::Borrow;

pub struct MVB<'l> {
    location: Location<'l, 'l, 'l>,
}

impl MVB<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "mvb",
                name: "M.V.B.",
                website: "https://mvb-leipzig.de/",
            },
        }
    }
}
impl Site for MVB<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let next_year = get_today().checked_add_months(Months::new(12)).unwrap();
        let next_year = next_year.format("%Y-%m-%d").to_string();

        // WordPress api of MyCalendar plugin
        // https://wordpress.org/plugins/my-calendar/
        let json = http
            .get_json(
                format!("https://mvb-leipzig.de/wp-json/my-calendar/v1/events?to={next_year}",)
                    .as_str(),
            )
            .unwrap();
        json.as_object()
            .unwrap()
            .iter()
            .for_each(|(date_key, val)| {
                val.as_array().unwrap().iter().for_each(|val| {
                    let data = val.as_object().unwrap();

                    let begin = data
                        .get("occur_begin")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    let end = data.get("occur_end").unwrap().as_str().unwrap().to_string();
                    let post_id = data
                        .get("event_post")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    let title = data
                        .get("event_title")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    let description = data
                        .get("event_desc")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    let image_url = data
                        .get("event_image")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    if begin.starts_with(date_key) && description.contains("Metal") {
                        let image = if image_url.ends_with("leerlogo.png") {
                            None
                        } else {
                            Some(image_url)
                        };

                        let mut evt = Event::new(
                            title,
                            parse_iso_datetime(begin.as_str()).unwrap(),
                            self.location.borrow(),
                            format!("{}?p={post_id}", self.location.website),
                            image,
                        );
                        evt.end_date = Some(parse_iso_datetime(end.as_str()).unwrap());
                        result.push(evt);
                    }
                })
            });

        result
    }
}
