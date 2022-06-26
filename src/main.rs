use crate::event::{Event, Location};
use crate::site::anker::Anker;
use crate::site::Site;
use crate::tools::image::optimize_image;
use crate::tools::HTTP;

mod event;
mod site;
mod tools;

fn main() {
    let http = HTTP::new();

    let mut locations: Vec<Location> = vec![];
    let mut events: Vec<Event> = vec![];

    let sites = [Anker::new()];
    for site in &sites {
        let mut evts = site.fetch_events();
        events.append(&mut evts);
        locations.append(&mut site.get_locations());
    }

    events.sort_by(|a, b| b.date.cmp(&a.date));
    locations.sort_by(|a, b| b.name.cmp(&a.name));

    for event in &mut events {
        match &mut event.image {
            None => {}
            Some(img) => optimize_image(img, &http),
        }
    }
}
