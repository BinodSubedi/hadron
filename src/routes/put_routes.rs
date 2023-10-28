use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};


#[derive(Debug,Clone,FromForm,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PutStandardInputFormat<'a>{
    data: &'a str,
}


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PutStandardResponse{
    
    status: u8,
    response: ResponseStatus


}


#[put("/<collection>",format = "json",data="<body>")]
pub async fn put_one(collection:String, body:Json<PutStandardInputFormat<'_>>)-> Json<PutStandardResponse>{
//pub async fn put_one(collection:String, form:Form<PutStandardInputFormat<'_>>)-> Json<PutStandardResponse>{
    //println!("{:?}",form);
     
    

//    println!("{:?}", data);
        
    //result
    //String::from("Sent")
    //

    let res = PutStandardResponse{
        status: 200,
        response: ResponseStatus::Success

    };

    Json(res)

}
