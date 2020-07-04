use rocket::request::Form;

#[get("/")]
pub fn index() -> &'static str {
    "Recipes"
}

#[derive(FromForm)]
pub struct Recipe {
    name: String,
    description: String
}

#[post("/recipes", data="<recipe>")]
pub fn create(recipe: Form<Recipe>) {
}
