pub mod admin;
pub mod pages;

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct TemplateContext<T> {
    items: Vec<T>,
}
