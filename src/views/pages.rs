use crate::auth::LoginForm;
use crate::{views::NoContext, Db, DreamResult};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(_connection: Db) -> Template {
    Template::render("pages/index", NoContext {})
}

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("login", NoContext {})
}

#[post("/login", data = "<login_form>")]
pub fn login(mut cookies: Cookies, login_form: Form<LoginForm>) -> DreamResult<Redirect> {
    if login_form.password == "42" {
        cookies.add_private(Cookie::new("admin", 1.to_string()));

        Ok(Redirect::to("/admin"))
    } else {
        Ok(Redirect::to("/login"))
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("admin"));

    Redirect::to("/")
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}
