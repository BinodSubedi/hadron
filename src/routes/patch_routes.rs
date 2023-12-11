use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;
use std::collections::HashMap;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 


#[derive(Serialize,Deserialize,Debug)]
#[serde(crate= "rocket::serde")]
pub struct PatchStandardInputFormat{
    
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
pub struct PatchStandardResponse{
    
    status: u16,
    response: ResponseStatus


}

#[delete("/<collection>",format = "json",data="<body>")]
pub async fn patch(collection:String, body:Json<PatchStandardInputFormat>)-> Json<PatchStandardResponse>{

    println!("{:?}",body);

    return Json(PatchStandardResponse{
        
        status: 200,
        response: ResponseStatus::Success

    });

}
