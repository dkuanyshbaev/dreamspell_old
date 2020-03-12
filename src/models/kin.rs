use super::schema::kins;
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
#[table_name = "kins"]
pub struct NewKin {
    pub num: i32,
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Kin {
    pub id: i32,
    pub num: i32,
    pub name: String,
    pub image: String,
}

impl Kin {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Kin>> {
        kins::table.order(kins::id.asc()).load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Kin> {
        kins::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_kin: NewKin) -> QueryResult<Kin> {
        diesel::insert_into(kins::table)
            .values(new_kin)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, mut new_kin: NewKin, id: i32) -> QueryResult<Kin> {
        let old_kin: Kin = Self::get(connection, id)?;
        if new_kin.image == "".to_string() {
            // keep old image name in case of update without image
            new_kin.image = old_kin.image.clone();
        } else {
            delete_file(&old_kin.image);
        }

        diesel::update(&old_kin).set(new_kin).get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Kin> {
        // remove related image
        let kin: Kin = Self::get(connection, id)?;
        delete_file(&kin.image);

        diesel::delete(&kin).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewKin {
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

        Success(NewKin {
            num,
            name: name.to_string(),
            image,
        })
    }
}
