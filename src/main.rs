#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod config;
mod errors;
mod models;
mod oracle;
mod views;

use config::Config;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use views::operator::{hexagrams, simulation, trigrams};
use views::{catchers, pages};

embed_migrations!();

#[database("ioracle")]
pub struct Db(diesel::SqliteConnection);

async fn run_migrations(mut rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = Db::get_one(rocket.inspect().await).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            println!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

#[launch]
fn rocket() -> Rocket {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Can't parsing config: {}", err);
        std::process::exit(1);
    });

    rocket::ignite()
        .manage(config)
        .attach(Db::fairing())
        .attach(AdHoc::on_attach("run migrations", run_migrations))
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::question,
                pages::answer,
                pages::operator,
                pages::run,
                pages::save,
                pages::load,
            ],
        )
        .mount(
            "/trigrams",
            routes![trigrams::all, trigrams::edit, trigrams::update,],
        )
        .mount(
            "/hexagrams",
            routes![hexagrams::all, hexagrams::edit, hexagrams::update,],
        )
        .mount(
            "/simulation",
            routes![simulation::touch, simulation::element, simulation::result],
        )
        .mount(
            "/test",
            routes![
                simulation::heaven_test_on,
                simulation::heaven_test_off,
                simulation::cloud_test_on,
                simulation::cloud_test_off,
                simulation::sun_test_on,
                simulation::sun_test_off,
                simulation::wind_test_on,
                simulation::wind_test_off,
                simulation::thunder_test_on,
                simulation::thunder_test_off,
                simulation::water_test_on,
                simulation::water_test_off,
                simulation::mountain_test_on,
                simulation::mountain_test_off,
                simulation::earth_test_on,
                simulation::earth_test_off,
            ],
        )
        .register(catchers![catchers::not_found, catchers::internal_error])
}
