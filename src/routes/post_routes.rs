use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;
use std::collections::HashMap;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostStandardResponse{
    
    status: u16,
    response: ResponseStatus


}




#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostStandardInputFormat{
    
    pub data: Vec<String>,

}


#[post("/<collection>",format = "json",data="<body>")]
pub async fn put_one(collection:String, body:Json<PostStandardInputFormat>)-> Json<PostStandardResponse>{


    return Json(PostStandardResponse{ 
        status : 200,
        response : ResponseStatus::Success 

    })

}
