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
    cityname: String,
    intro: String,
    spotcats: Vec<SpotCat>,
    eating: Vec<Eating>,
    things: Vec<Thing>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thing {
    title: String,
    paragraphs: Vec<String>,
    nicelinks: Vec<NiceLink>,
}

impl Thing {
    fn new() -> Thing {
        Thing {
            title: "Hiking".to_string(),
            paragraphs: vec!["Hiking in WA is simply amazing. Trails of all difficulties can be found less than an hour away, and you can travel as far as you'd like to get some more unique views.".to_string(), "Alpine lakes, scenic mountain views, beautiful forests all await you.".to_string()],
            nicelinks: vec![NiceLink::new(), NiceLink::new()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Eating {
    name: String,
    category: String,
    description: String,
    area: String,
    urls: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    show: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NiceLink {
    link: Link,
    description: String,
}

impl NiceLink {
    fn new() -> Self {
        NiceLink {
            link: Link {
                show: "wta.org".to_string(),
                url: "https://wta.org".to_string(),
            },
            description: "Washington Trails Association - Best site for hiking!!".to_string(),
        }
    }
}

impl Link {
    fn new() -> Link {
        Link {
            show: "yelp".to_string(),
            url: "#".to_string(),
        }
    }
}

impl Eating {
    fn new() -> Eating {
        Eating {
            name: "Cafe Vita".to_string(),
            category: "Coffee".to_string(),
            description: " Super hip & awesome vibe, in the heart of Cap Hilla dasdfasdfasd s a as asd asd asd asd".to_string(),
            area: "Multiple, Cap Hill".to_string(),
            urls: vec![Link::new(), Link::new(), Link::new()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotCat {
    title: String,
    description: String,
    spots: Vec<Spot>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Spot {
    name: String,
    text: Vec<String>,
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
    map.insert("seattle", CityData {
            cityname: "Seattle".to_string(),
            intro: "The emerald city".to_string(),
            eating: vec![Eating::new(), Eating::new()],
            spotcats: vec![
                SpotCat {
                    title: "FirstCat".to_string(),
                    description: "FirstDesc".to_string(),
                    spots: vec![
                        Spot {
                            name: "SpotName".to_string(),
                            text: vec!["Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborums".to_string(), "this is another thing".to_string()],
                        },
                        Spot {
                            name: "OtherSpot".to_string(),
                            text: vec!["Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborums".to_string()],
                        }
                    ]
                },
                SpotCat {
                    title: "SecondCategory".to_string(),
                    description: "asdfasdfasdf".to_string(),
                    spots: vec![
                        Spot {
                            name: "this is a spot".to_string(),
                            text: vec!["surmolm".to_string()],
                        }
                    ]
                }
            ],
            things: vec![Thing::new(), Thing::new(), Thing::new()]
        });

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