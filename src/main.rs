use std::borrow::Borrow;
use std::collections::BTreeMap;

use chrono::{Days, NaiveDate, Timelike};

use crate::event::{Event, Location};
use crate::site::anker::Anker;
use crate::site::arena::Arena;
use crate::site::bandcommunity::Bandcommunity;
use crate::site::conne_island::ConneIsland;
use crate::site::felsenkeller::Felsenkeller;
use crate::site::haus_auensee::HausAuensee;
use crate::site::hellraiser::Hellraiser;
use crate::site::inflammen::InFlammen;
use crate::site::moritzbastei::Moritzbastei;
use crate::site::noels::NoelsBallroom;
use crate::site::parkbuehne::Parkbuehne;
use crate::site::taeubchenthal::Taeubchenthal;
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

    let yesterday = chrono::Utc::now()
        .checked_sub_days(Days::new(1))
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .naive_local();

    let mut locations: Vec<&Location> = vec![];
    let mut events: Vec<Event> = vec![];

    let sites: Vec<Box<dyn Site>> = vec![
        Box::new(Anker::new()),
        Box::new(Arena::new_festwiese()),
        Box::new(Arena::new_quarterback()),
        Box::new(Arena::new_red_bull()),
        Box::new(Bandcommunity::new()),
        Box::new(ConneIsland::new(insecure_http.borrow())),
        Box::new(Felsenkeller::new()),
        Box::new(HausAuensee::new()),
        Box::new(Hellraiser::new()),
        Box::new(InFlammen::new()),
        Box::new(Moritzbastei::new()),
        Box::new(NoelsBallroom::new()),
        Box::new(Parkbuehne::new(insecure_http.borrow())),
        //Box::new(Soltmann::new()),
        Box::new(Taeubchenthal::new()),
        Box::new(UTConnewitz::new()),
        Box::new(Werk2::new()),
    ];
    for site in &sites {
        let mut evts: Vec<Event> = site
            .fetch_events(http.borrow())
            .into_iter()
            .filter(|evt| evt.start_date.gt(&yesterday))
            .collect();

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

    let grouped_events = {
        let mut grouped_events: BTreeMap<NaiveDate, Vec<Event>> = BTreeMap::new();

        for event in events {
            let mut date = event.start_date;
            while date <= event.end_date.unwrap_or(event.start_date) {
                grouped_events
                    .entry(date.date())
                    .or_insert(Vec::new())
                    .push(event.clone());
                date = date
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .with_hour(8)
                    .unwrap()
                    .with_minute(0)
                    .unwrap();
            }
        }

        grouped_events
    };

    renderer::render(grouped_events, locations);
}
