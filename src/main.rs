#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{Template, Engines};

struct GlobalState {
    cities: HashMap<&'static str, CityData>,
}

impl GlobalState {
    fn new(map: HashMap<&'static str, CityData>) -> Self {
        GlobalState {
            cities: map,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CityData {
    num: usize,
}

#[get("/<city>")]
fn city(city: String, state: State<GlobalState>) -> Option<Template> {
    let lower = city.to_lowercase();
    if let Some(citydata) =  state.cities.get(&lower.as_str()) {
        Some(Template::render("city", &citydata))
    } else {
        None
    }
}

#[get("/")]
fn index() -> Template {
    Template::render("index", ())
}

fn main() {
    let mut map = HashMap::new();
    map.insert("seattle", CityData { num: 3 }); // Here is where we load up our things!!

    let template_fairing = Template::custom(|engines: &mut Engines| {
        engines.handlebars.set_strict_mode(false);
    });

    rocket::ignite()
        .attach(template_fairing)
        .manage(GlobalState::new(map))
        .mount("/resources", StaticFiles::from("resources"))
        .mount("/", routes![index, city])
        .launch();
}