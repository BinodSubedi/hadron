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

#[delete("/<collection>/<id>",format = "json",data="<body>")]
pub async fn delete_one(collection:String,id:String, body:Json<DeleteStandardInputFormat>)-> Json<DeleteStandardResponse>{

    println!("{:?}",body);
    println!("{}", id);

    //Steps are simple
    //First implement all logic in get 
    //except we need to stop at the file where we find the match and
    //stop at the same point, appending a new member data in the file where we removed the data
    //and the data comes from last member of collection and //NOTE:: we need to delete the last
    //file if the last file has no data any longer //ALSO:: If the data removed file is last do
    //NOTHING



    return Json(DeleteStandardResponse{
        
        status: 200,
        response: ResponseStatus::Success

    });

}
