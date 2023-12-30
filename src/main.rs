use std::borrow::Borrow;

use chrono::NaiveDate;

use crate::event::{Event, Location};
use crate::site::anker::Anker;
use crate::site::arena::Arena;
use crate::site::bandcommunity::Bandcommunity;
use crate::site::conne_island::ConneIsland;
use crate::site::darkflower::Darkflower;
use crate::site::felsenkeller::Felsenkeller;
use crate::site::haus_auensee::HausAuensee;
use crate::site::hellraiser::Hellraiser;
use crate::site::Site;
use crate::tools::image::optimize_image;
use crate::tools::HTTP;

mod event;
mod renderer;
mod site;
mod tools;

fn main() {
    let http = HTTP::new();

    let mut locations: Vec<Location> = vec![];
    let mut events: Vec<Event> = vec![];

    let sites: Vec<Box<dyn Site>> = vec![
        Box::new(Anker::new()),
        Box::new(Arena::new_red_bull()),
        Box::new(Arena::new_quarterback()),
        Box::new(Arena::new_festwiese()),
        Box::new(Bandcommunity::new()),
        Box::new(ConneIsland::new()),
        Box::new(Darkflower::new()),
        Box::new(Felsenkeller::new()),
        Box::new(HausAuensee::new()),
        Box::new(Hellraiser::new()),
    ];
    for site in &sites {
        let mut evts = site.fetch_events();
        events.append(&mut evts);
        locations.append(&mut site.get_locations());
    }

    events.sort_by(|a, b| a.date.cmp(&b.date));
    locations.sort_by(|a, b| b.name.cmp(&a.name));

    for event in &mut events {
        match &mut event.image {
            None => {}
            Some(img) => optimize_image(img, &http),
        }
    }

    let mut grouped_events: Vec<Vec<Event>> = vec![];
    {
        let mut previous_date: NaiveDate = NaiveDate::from_ymd(1970, 1, 1);
        for event in events {
            if event.date.date().cmp(previous_date.borrow()).is_gt() {
                grouped_events.push(Vec::new());
            }

            previous_date = event.date.date();
            grouped_events.last_mut().unwrap().push(event);
        }
    }

    renderer::render(grouped_events, locations);
}
