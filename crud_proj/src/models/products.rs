use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
/*
Product Model
 - `categories`: Category to which the product belongs
 - `brand`: Name of the product owner (`Option<String>`)
 - `image_url`: URL of the product image (`Option<String>`)
 - `serving_size_grams`: Serving size in grams
 - `is_upf`: Whether the product is ultra-processed food
 - `is_healthier`: Whether the product is certified [Healthier Choice](http://healthierlogo.com/)
 */

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub image_url: Option<String>,

    pub serving_size_grams: Option<f32>,
    pub calories: i32,
    pub fat: f32,
    pub sugar: f32,
    pub sodium: f32,
    pub protein: f32,
    pub carbs: f32,

    pub saturated_fat: f32,
    pub cholesterol: f32,
    pub vitamin_c: Option<f32>,
    pub calcium: Option<f32>,
    pub vitamin_b1: Option<f32>,
    pub vitamin_a: Option<f32>,

    pub price: f32,
    pub is_upf: bool,
    pub is_healthier: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductForm {
    pub id: Option<String>,
    pub name: String,
    pub brand: Option<String>,
    pub image_url: Option<String>,
    pub categories_ids: Vec<String>,
    pub serving_size_grams: Option<f32>,
    pub calories: i32,
    pub fat: f32,
    pub sugar: f32,
    pub sodium: f32,
    pub protein: f32,
    pub carbs: f32,
    pub saturated_fat: f32,
    pub cholesterol: f32,
    pub vitamin_c: Option<f32>,
    pub calcium: Option<f32>,
    pub vitamin_b1: Option<f32>,
    pub vitamin_a: Option<f32>,
    pub price: f32,
    pub is_upf: bool,
    pub is_healthier: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub image_url: Option<String>,
    pub categories: Vec<String>,

    pub serving_size_grams: Option<f32>,
    pub calories: i32,
    pub fat: f32,
    pub sugar: f32,
    pub sodium: f32,
    pub protein: f32,
    pub carbs: f32,

    pub saturated_fat: f32,
    pub cholesterol: f32,
    pub vitamin_c: Option<f32>,
    pub calcium: Option<f32>,
    pub vitamin_b1: Option<f32>,
    pub vitamin_a: Option<f32>,

    pub price: f32,
    pub is_upf: bool,
    pub is_healthier: bool,
}
