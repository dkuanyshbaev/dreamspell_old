#[get("/")]
pub fn main() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/admin/glyphs")
}

macro_rules! handle {
    ($t:ty, $nt:ty, $tp:expr) => {
        // list of items
        #[get("/")]
        pub fn list(
            connection: crate::Db,
        ) -> crate::DreamResult<rocket_contrib::templates::Template> {
            let items = <$t>::all(&connection)?;
            let context: crate::views::TemplateContext<$t> =
                crate::views::TemplateContext { items };

            Ok(rocket_contrib::templates::Template::render(
                format!("{}/list", $tp),
                context,
            ))
        }

        // show add form
        #[get("/add")]
        pub fn add() -> rocket_contrib::templates::Template {
            rocket_contrib::templates::Template::render(
                format!("{}/add", $tp),
                crate::views::NoContext {},
            )
        }

        // create item
        #[post("/", data = "<new_item>")]
        pub fn create(
            connection: crate::Db,
            new_item: crate::DreamResult<$nt>,
        ) -> crate::DreamResult<rocket::response::Redirect> {
            match new_item {
                Ok(item) => {
                    let _item = <$t>::insert(&connection, item)?;
                    Ok(rocket::response::Redirect::to(format!("/{}", $tp)))
                }
                Err(error) => {
                    println!("Error: {}", error);
                    Ok(rocket::response::Redirect::to(format!("/{}/add", $tp)))
                }
            }
        }

        // show edit form
        #[get("/<id>")]
        pub fn edit(
            connection: crate::Db,
            id: i32,
        ) -> crate::DreamResult<rocket_contrib::templates::Template> {
            let item = <$t>::get(&connection, id)?;

            Ok(rocket_contrib::templates::Template::render(
                format!("{}/edit", $tp),
                item,
            ))
        }

        // update item
        // post here instead of put - because of multipart
        #[post("/<id>", data = "<new_item>")]
        pub fn update(
            connection: crate::Db,
            new_item: crate::DreamResult<$nt>,
            id: i32,
        ) -> crate::DreamResult<rocket::response::Redirect> {
            match new_item {
                Ok(item) => {
                    let _item = <$t>::update(&connection, item, id)?;
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }

            Ok(rocket::response::Redirect::to(format!("/{}/{}", $tp, id)))
        }

        // delete item
        #[delete("/<id>")]
        pub fn delete(
            connection: crate::Db,
            id: i32,
        ) -> crate::DreamResult<rocket::response::Redirect> {
            let _item = <$t>::delete(&connection, id)?;

            Ok(rocket::response::Redirect::to(format!("/{}", $tp)))
        }
    };
}

pub mod glyphs {
    use crate::models::glyph::{Glyph, NewGlyph};
    handle!(Glyph, NewGlyph, "admin/glyphs");
}

pub mod tones {
    use crate::models::tone::{NewTone, Tone};
    handle!(Tone, NewTone, "admin/tones");
}

pub mod kins {
    use crate::models::kin::{Kin, NewKin};
    handle!(Kin, NewKin, "admin/kins");
}
