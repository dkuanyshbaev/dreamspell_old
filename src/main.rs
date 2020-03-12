#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate rocket_multipart_form_data;

use diesel::PgConnection;
use rocket::Rocket;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use views::{admin, pages};

mod auth;
mod errors;
mod models;
mod views;

type DreamResult<T> = Result<T, errors::DreamError>;

#[database("dreamspell")]
pub struct Db(PgConnection);

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![pages::index, pages::login_page, pages::login, pages::logout],
        )
        .mount("/admin", routes![admin::main])
        .mount(
            "/admin/glyphs",
            routes![
                admin::glyphs::list,
                admin::glyphs::add,
                admin::glyphs::create,
                admin::glyphs::edit,
                admin::glyphs::update,
                admin::glyphs::delete,
            ],
        )
        .mount(
            "/admin/tones",
            routes![
                admin::tones::list,
                admin::tones::add,
                admin::tones::create,
                admin::tones::edit,
                admin::tones::update,
                admin::tones::delete,
            ],
        )
        .mount(
            "/admin/kins",
            routes![
                admin::kins::list,
                admin::kins::add,
                admin::kins::create,
                admin::kins::edit,
                admin::kins::update,
                admin::kins::delete,
            ],
        )
        .register(catchers![pages::not_found, pages::unauthorized])
}

fn main() {
    let error = rocket().launch();
    println!("Launch failed: {}", error);
}
