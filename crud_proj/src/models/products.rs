use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
/*
Model ของ Categoris ที่ใช้จัดหมวดหมู่ของ Product
*/

// pub struct Categories {
//     pub id: String,
//     pub name: String,
// }


/* 
Model ของ Product
categoris ใช้อิงว่าอาหารนี้อยู่หมวดอะไร
brand และใช้ Option<String> เพื่อรองรับกรณีที่ไม่มีข้อมูล
image_url และใช้ Option<String>
serving_size_grams ขนาดการบริโภคต่อหน่วยเป็นกรัม
is_upf เป็น Ultra process food หรือไม่
is_healthier ผ่านเกณฑ์การรับรองตาม http://healthierlogo.com/ หรือไม่
*/
#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct Product {
    pub id:Uuid,
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
    pub is_healthier:bool

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductForm {
    pub id: Option<String>,
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
    pub is_healthier:bool

}