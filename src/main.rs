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
use crate::site::moritzbastei::Moritzbastei;
use crate::site::parkbuehne::Parkbuehne;
use crate::site::soltmann::Soltmann;
use crate::site::taeubchenthal::Taeubchenthal;
use crate::site::tankbar::Tankbar;
use crate::site::ut_connewitz::UTConnewitz;
use crate::site::werk2::Werk2;
use crate::site::Site;
use crate::tools::image::optimize_image;
use crate::tools::Http;

mod event;
mod renderer;
mod site;
mod tools;

fn main() {
    let http = Http::new(false);
    let insecure_http = Http::new(true);

    let mut locations: Vec<&Location> = vec![];
    let mut events: Vec<Event> = vec![];

    let sites: Vec<Box<dyn Site>> = vec![
        Box::new(Anker::new()),
        Box::new(Arena::new_festwiese()),
        Box::new(Arena::new_quarterback()),
        Box::new(Arena::new_red_bull()),
        Box::new(Bandcommunity::new()),
        Box::new(ConneIsland::new(insecure_http.borrow())),
        Box::new(Darkflower::new()),
        Box::new(Felsenkeller::new()),
        Box::new(HausAuensee::new()),
        Box::new(Hellraiser::new()),
        Box::new(Moritzbastei::new()),
        Box::new(Parkbuehne::new(insecure_http.borrow())),
        Box::new(Soltmann::new()),
        Box::new(Taeubchenthal::new()),
        Box::new(Tankbar::new()),
        Box::new(UTConnewitz::new()),
        Box::new(Werk2::new()),
    ];
    for site in &sites {
        let mut evts = site.fetch_events(http.borrow());
        if !evts.is_empty() {
            locations.push(site.get_location());

            for event in &mut evts {
                match &mut event.image {
                    None => {}
                    Some(img) => optimize_image(img, &http),
                }
            }
            events.append(&mut evts);
        }
    }

    events.sort_by(|a, b| a.start_date.cmp(&b.start_date));
    locations.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let mut grouped_events: Vec<Vec<Event>> = vec![];
    {
        let mut previous_date: NaiveDate = NaiveDate::from_ymd(1970, 1, 1);
        for event in events {
            if event.start_date.date().cmp(previous_date.borrow()).is_gt() {
                grouped_events.push(Vec::new());
            }

            previous_date = event.start_date.date();
            grouped_events.last_mut().unwrap().push(event);
        }
    }

    renderer::render(grouped_events, locations);
}
