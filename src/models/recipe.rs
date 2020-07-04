use std;
use std::io;
use bson;
use mongodb;
use crate::lib;

pub struct Recipe {
    pub name: String,
    pub description: String
}

impl Recipe {
    pub async fn save(&self) {
        let client = lib::mongo::establish_connection().await.unwrap();
        let collection = client.database("food_recipes").collection("recipes");

        collection.insert_one(
            bson::doc! {"name": &self.name, "description": &self.description},
            None
        ).await;
    }
}
