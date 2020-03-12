use super::schema::glyphs;
use super::utils::{delete_file, file_name_with_prefix, save_file};
use crate::errors::DreamError;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
};

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "glyphs"]
pub struct NewGlyph {
    pub num: i32,
    pub name: String,
    pub image: String,
    pub preview: String,
    pub description: String,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Glyph {
    pub id: i32,
    pub num: i32,
    pub name: String,
    pub image: String,
    pub preview: String,
    pub description: String,
}

impl Glyph {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Glyph>> {
        glyphs::table.order(glyphs::id.asc()).load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Glyph> {
        glyphs::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_glyph: NewGlyph) -> QueryResult<Glyph> {
        diesel::insert_into(glyphs::table)
            .values(new_glyph)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_glyph: NewGlyph,
        id: i32,
    ) -> QueryResult<Glyph> {
        let old_glyph: Glyph = Self::get(connection, id)?;
        if new_glyph.image == "".to_string() {
            // keep old image name in case of update without image
            new_glyph.image = old_glyph.image.clone();
        } else {
            delete_file(&old_glyph.image);
        }

        diesel::update(&old_glyph)
            .set(new_glyph)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Glyph> {
        // remove related image
        let glyph: Glyph = Self::get(connection, id)?;
        delete_file(&glyph.image);

        diesel::delete(&glyph).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewGlyph {
    type Error = DreamError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("image"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("num"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("name"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("preview"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("description"));

        // check if the content type is set properly
        let content_type = match request.content_type() {
            Some(content_type) => content_type,
            _ => {
                return Failure((Status::BadRequest, DreamError::BadRequest));
            }
        };

        // do the form parsing and return on error
        let multipart_form = match MultipartFormData::parse(&content_type, data, options) {
            Ok(multipart) => multipart,
            Err(error) => {
                println!("Multipart form parsing error: {:?}", error);
                return Failure((Status::BadRequest, DreamError::BadRequest));
            }
        };

        let mut image = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("image") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    image = file_name_with_prefix(file_path);
                    save_file(path, &image);
                }
            }
        }

        let mut num = 0;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("num") {
            let amount = &text.text;
            num = amount.parse().unwrap();
        }

        let mut name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            name = &text.text;
        }

        let mut preview = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("preview") {
            preview = &text.text;
        }

        let mut description = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("description") {
            description = &text.text;
        }

        Success(NewGlyph {
            num,
            name: name.to_string(),
            image,
            preview: preview.to_string(),
            description: description.to_string(),
        })
    }
}
