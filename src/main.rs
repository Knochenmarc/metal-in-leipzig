use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::env;

use chrono::{Days, NaiveDate, NaiveTime, Timelike};

use crate::event::{Event, Location};
use crate::site::anker::Anker;
use crate::site::arena::Arena;
use crate::site::bandcommunity::Bandcommunity;
use crate::site::conne_island::ConneIsland;
use crate::site::darkaffair::DarkAffair;
use crate::site::darkflower::Darkflower;
use crate::site::felsenkeller::Felsenkeller;
use crate::site::festivals::Festivals;
use crate::site::forum::ZeitgeschichtlichesForum;
use crate::site::haus_auensee::HausAuensee;
use crate::site::hellraiser::Hellraiser;
use crate::site::moritzbastei::Moritzbastei;
use crate::site::muehlkeller::Muehlkeller;
use crate::site::noels::NoelsBallroom;
use crate::site::soltmann::Soltmann;
use crate::site::taeubchenthal::Taeubchenthal;
use crate::site::ut_connewitz::UTConnewitz;
// use crate::site::wavegothictreffen::WaveGothicTreffen;
use crate::site::werk2::Werk2;
use crate::site::Site;
use crate::tools::image::optimize_image;
use crate::tools::Http;

mod event;
mod renderer;
mod site;
mod tools;

const BLOCKLIST: &[&str] = &["2025-03-25-ha-AVATAR", "2025-04-19-ha-JAN & HENRY"];

fn parse_args(sites: Vec<Box<dyn Site>>) -> Vec<Box<dyn Site>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for site in sites {
            if site.get_location().slug.eq(args.get(1).unwrap()) {
                return vec![site];
            }
        }

        vec![]
    } else {
        sites
    }
}

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let http = Http::new(false);

    let today = chrono::Utc::now()
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
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
        Box::new(ConneIsland::new(Http::new(true))),
        Box::new(DarkAffair::new()),
        Box::new(Darkflower::new()),
        Box::new(Festivals::new()),
        Box::new(Felsenkeller::new()),
        Box::new(HausAuensee::new_auensee()),
        Box::new(HausAuensee::new_park()),
        Box::new(Hellraiser::new()),
        Box::new(Moritzbastei::new()),
        Box::new(Muehlkeller::new()),
        Box::new(NoelsBallroom::new()),
        Box::new(Soltmann::new()),
        Box::new(Taeubchenthal::new()),
        Box::new(UTConnewitz::new()),
        // Box::new(WaveGothicTreffen::new()),
        Box::new(Werk2::new()),
        Box::new(ZeitgeschichtlichesForum::new()),
    ];

    let sites = parse_args(sites);

    for site in &sites {
        let mut evts: Vec<Event> = site
            .fetch_events(http.borrow())
            .into_iter()
            .filter(|evt| {
                evt.start_date.ge(today.borrow())
                    || evt.end_date.map(|e| e.ge(today.borrow())).unwrap_or(false)
            })
            .collect();

        if evts.is_empty() {
            println!(
                "::warning::Keine Events bei {} gefunden. Bitte prüfen.",
                site.get_location().name
            );
        } else {
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

    events.retain(|evt| {
        !BLOCKLIST.contains(
            &format!(
                "{}-{}-{}",
                evt.start_date.format("%Y-%m-%d"),
                evt.location.slug,
                evt.name
            )
            .as_str(),
        )
    });

    events.sort_by(|a, b| a.start_date.cmp(&b.start_date));
    locations.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let grouped_events = {
        let mut grouped_events: BTreeMap<NaiveDate, Vec<Event>> = BTreeMap::new();

        for event in events {
            let mut date = event.start_date;
            while date <= event.end_date.unwrap_or(event.start_date) {
                grouped_events
                    .entry(date.date())
                    .or_default()
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
