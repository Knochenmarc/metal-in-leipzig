use crate::site::anker::Anker;
use crate::site::Site;

mod event;
mod site;
mod tools;

fn main() {
    let sites = [Anker::new()];
    for site in sites {
        site.fetch_events();
    }
}
