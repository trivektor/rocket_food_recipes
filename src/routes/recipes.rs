#[get("/")]
pub fn index() -> &'static str {
    "Recipes"
}
