use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;
use std::collections::HashMap;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 


#[derive(Serialize,Deserialize,Debug)]
#[serde(crate= "rocket::serde")]
pub struct DeleteStandardInputFormat{
    
    data: Value
}


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct DeleteStandardResponse{
    
    status: u16,
    response: ResponseStatus


}

#[delete("/<collection>",format = "json",data="<body>")]
pub async fn delete_one(collection:String, body:Json<DeleteStandardInputFormat>)-> Json<DeleteStandardResponse>{

    println!("{:?}",body);

    return Json(DeleteStandardResponse{
        
        status: 200,
        response: ResponseStatus::Success

    });

}
